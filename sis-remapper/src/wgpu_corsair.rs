use std::{borrow::Cow, error::Error, os::raw::c_void, sync::mpsc::{self, Receiver, Sender}, time::{Duration, Instant}};

use icue_bindings::{types::{CorsairDeviceId, CorsairDeviceType, CorsairLedColor, CorsairLedLuid, CorsairSessionState}, CorsairConnect, CorsairGetDevices, CorsairGetLedPositions, CorsairSetLedColors};
use wave_effects::{ripple_params, wave_params};
use wgpu::{core::instance, hal::auxil::db, util::{BufferInitDescriptor, DeviceExt}, Backends, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, Buffer, BufferDescriptor, BufferUsages, CommandEncoder, CommandEncoderDescriptor, ComputePassDescriptor, ComputePipeline, ComputePipelineDescriptor, Device, DeviceDescriptor, Features, InstanceDescriptor, Maintain, MaintainBase, MapMode, PushConstantRange, Queue, RequestAdapterOptions, ShaderModuleDescriptor, ShaderSource, ShaderStages};

use crate::corsair::{corsair_connect, effects::{CorsairLedColorf32, Effect}, CorsairMsg};

use self::wave_effects::{floatled_to_colorled, ripple_effect, static_effect, wave_effect, LedInfof32};

static mut STATE: CorsairSessionState = CorsairSessionState::Invalid;

pub(crate) mod wave_effects;

#[cfg(feature = "testable_privates")]
pub mod test_exposer;

pub(crate) fn init_corsair() -> Sender<CorsairMsg> {
    let (tx, rx) = corsair_connect();
    std::thread::spawn(||listener(rx));

    tx
}

fn wait_connection(corsair_state: &mut CorsairState, rx: &Receiver<CorsairMsg>, connected: &mut bool) {
    while let Ok(msg) = rx.recv() {
        corsair_state.handle_msg(connected, msg);
        if *connected {
            break;
        }
    }
}

fn listener(rx: Receiver<CorsairMsg>) {
    let mut connected = false;
    let mut corsair_state = CorsairState::new();
    loop {
        if !connected {
            wait_connection(&mut corsair_state, &rx, &mut connected);
            connected = true;
            corsair_state.setup()
        } else {
            match rx.try_recv() {
                Ok(msg) => corsair_state.handle_msg(&mut connected, msg),
                Err(err) => match err {
                    mpsc::TryRecvError::Empty => corsair_state.tick(),
                    mpsc::TryRecvError::Disconnected => panic!("Channel closed"),
                },
            }
        }
    }
}

struct Pipeline {
    pipeline: ComputePipeline,
    bg: Option<BindGroup>,
}

struct Pipelines {
    stat: Pipeline,
    ripple: Pipeline,
    wave: Pipeline,
}

struct WgpuState {
    device: Device,
    queue: Queue,
    dst_buffer: Option<Buffer>,
    src_buffer: Option<Buffer>,
    pipelines: Pipelines,
    data_size: u64
}
impl WgpuState {
    fn new() -> Self {
        let instance = wgpu::Instance::new(InstanceDescriptor {
            ..Default::default()
        });

        let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
            ..Default::default()
        })).unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(&DeviceDescriptor {
            required_features: Features::default() | Features::PUSH_CONSTANTS,
            ..Default::default()
        }, None)).unwrap();

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("static_effect.wgsl"))),
        }).unwrap_or_else(|e| panic!("{}", e));

        let bgl = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None,
                }
            ],
        }).unwrap();

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bgl],
            push_constant_ranges: &[]
        }).unwrap();

        let static_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: "static_effect",
        }).unwrap();

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("ripple_effect.wgsl"))),
        }).unwrap_or_else(|e| {dbg!(&e);panic!("{}", e)});

        let bgl = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None,
                },
            ],
        }).unwrap();
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bgl],
            push_constant_ranges: &[]
        }).unwrap();

        let ripple_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: "ripple_effect",
        }).unwrap();

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("wave_effect.wgsl"))),
        }).unwrap_or_else(|e| {dbg!(&e);panic!("{}", e)});

        let bgl = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None,
                },
            ],
        }).unwrap();
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bgl],
            push_constant_ranges: &[]
        }).unwrap();

        let wave_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: "wave_effect",
        }).unwrap();

        let pipelines = Pipelines {
            stat: Pipeline{
                pipeline: static_pipeline,
                bg: None,
            },
            ripple: Pipeline{
                pipeline: ripple_pipeline,
                bg: None,
            },
            wave: Pipeline{
                pipeline: wave_pipeline,
                bg: None,
            },
        };

        WgpuState {
            device,
            queue,
            dst_buffer: None,
            src_buffer: None,
            data_size: 0,
            pipelines,
        }
    }

    fn get_led_colors(&self, effects: &[Effect], dt: u64) -> Vec<[f32;4]> {
        let src_buffer = self.src_buffer.as_ref().unwrap();
        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor { label: None }).unwrap();
        Self::compute_pass(&mut encoder, &self.pipelines, self.data_size, effects, dt);

        encoder.copy_buffer_to_buffer(self.src_buffer.as_ref().unwrap(), 0, self.dst_buffer.as_ref().unwrap(), 0, self.data_size);

        self.queue.submit(Some(encoder.finish().unwrap()));

        let buffer_slice = self.dst_buffer.as_ref().unwrap().slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        buffer_slice.map_async(MapMode::Read, move |v| tx.send(v).unwrap()).unwrap();

        self.device.poll(Maintain::wait()).panic_on_timeout();

        rx.recv().unwrap().unwrap();
        let data = buffer_slice.get_mapped_range();
        let result = bytemuck::cast_slice(&data).to_vec();
        std::mem::drop(data);
        src_buffer.unmap().unwrap();

        result
    }

    fn compute_pass(encoder: &mut CommandEncoder, pipelines: &Pipelines, data_size: u64, effects: &[Effect], dt: u64) {
        let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: None,
            timestamp_writes: None,
        });

        for effect in effects {
            match effect {
                Effect::Static(color) => {
                    pass.set_pipeline(&pipelines.stat.pipeline);
                    pass.set_bind_group(
                        0,
                        pipelines.stat.bg.as_ref().unwrap(),
                        &[]
                    );
                    pass.set_push_constants(0, bytemuck::cast_slice(color))
                },
                Effect::Wave(wave) => {
                    pass.set_pipeline(&pipelines.wave.pipeline);
                    pass.set_bind_group(
                        0,
                        pipelines.wave.bg.as_ref().unwrap(),
                        &[]
                    );
                    let params = wave_params(dt, wave);
                    pass.set_push_constants(0, params.bytes())
                },
                Effect::Ripple(ripple) => {
                    pass.set_pipeline(&pipelines.ripple.pipeline);
                    pass.set_bind_group(
                        0,
                        pipelines.ripple.bg.as_ref().unwrap(),
                        &[]
                    );
                    let params = ripple_params(dt, ripple);
                    pass.set_push_constants(0, params.bytes())
                },
                Effect::ColorChange => todo!(),
            }
        }

        pass.dispatch_workgroups(data_size as u32, 1, 1);
    }

    fn setup(&mut self, leds: Vec<[f32;4]>) {
        self.data_size = leds.len() as u64;

        let dst_buffer = self.device.create_buffer(&BufferDescriptor {
            label: None,
            size: self.data_size,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false
        }).unwrap();

        let src_buffer = self.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&leds),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
        }).unwrap();

        let bgl = self.pipelines.stat.pipeline.get_bind_group_layout(0);
        let bg = self.device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &bgl,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: src_buffer.as_entire_binding(),
                }
            ],
        }).unwrap();
        self.pipelines.stat.bg = Some(bg);

        let bgl = self.pipelines.ripple.pipeline.get_bind_group_layout(0);
        let bg = self.device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &bgl,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: src_buffer.as_entire_binding(),
                },
            ],
        }).unwrap();
        self.pipelines.ripple.bg = Some(bg);

        let bgl = self.pipelines.wave.pipeline.get_bind_group_layout(0);
        let bg = self.device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &bgl,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: src_buffer.as_entire_binding(),
                }
            ],
        }).unwrap();
        self.pipelines.wave.bg = Some(bg);

        self.dst_buffer = Some(dst_buffer);
        self.src_buffer = Some(src_buffer);
    }
}

struct CorsairState {
    start_time: Instant,
    keyboard_id: Option<CorsairDeviceId>,
    leds: Vec<LedInfof32>,
    effects: Vec<Effect>,
    key_effects: Vec<(CorsairLedLuid, Effect)>,
    wgpu: WgpuState
}

impl CorsairState {
    fn new() -> CorsairState {
        CorsairState {
            start_time: Instant::now(),
            keyboard_id: None,
            leds: Vec::new(),
            effects: Vec::new(),
            key_effects: Vec::new(),
            wgpu: WgpuState::new()
        }
    }

    fn setup(&mut self) {
        self.start_time = Instant::now();
        unsafe {
            let devices = CorsairGetDevices().unwrap();
            for device in devices {
                println!("Device found:");
                dbg!(&device);
                if device.type_ == CorsairDeviceType::Keyboard {
                    self.keyboard_id = Some(device.id)
                }
            }

            // TODO: Do Option<String> instead of String for this reason
            if let Some(id) = &self.keyboard_id {
                self.leds = CorsairGetLedPositions(id).unwrap().into_iter().map(|led| {
                    (
                        (led.cx, led.cy),
                        CorsairLedColorf32 {
                            id: led.id,
                            color: [0.0, 0.0, 0.0, 1.0]
                        }
                    )
                }).collect();
            }
        }
        self.wgpu.setup(self.leds.iter().map(|(_, c)| c.color).collect::<Vec<_>>());
    }

    fn get_led_colors(&self) -> Vec<CorsairLedColor> {
        // TODO: Improve performance. Too many clones
        let dt = self.start_time.elapsed().as_millis() as u64;
        //let nanos = self.start_time.elapsed().as_nanos() as u64;

        let leds = self.wgpu.get_led_colors(&self.effects, dt);

        /* for (key, effect) in self.key_effects.iter() { // TODO: rewrite this part so it works. Cannot be easily done when mutating leds
            let effect: Box<dyn Fn(LedInfof32) -> LedInfof32> = match effect {
                Effect::Static(color) => Box::new(move |key| static_key(key, color.clone())),
                Effect::Wave(wave) => Box::new(move |key| wave_key(key, dt, wave)),
                Effect::Ripple(ripple) => Box::new(move |key| ripple_key(key, dt, ripple)),
                Effect::ColorChange => Box::new(move |key| key),
            };

            leds = Box::new(leds.map(move |led| {
                if led.1.id == *key {
                    effect(led)
                } else {
                    led
                }
            }))
        } */

        floatled_to_colorled(&self.leds.iter().zip(leds).map(|(((pos), c), color)| {
            (*pos, CorsairLedColorf32 {
                id: c.id,
                color: color,
            })
        }).collect::<Vec<_>>()).map(|(_, led)| led).collect()
    }

    fn tick(&mut self) {
        if let Some(keyboard_id) = &self.keyboard_id {
            let leds = self.get_led_colors();
            unsafe {
                CorsairSetLedColors(keyboard_id, leds).unwrap();
            }
            // TODO: Allow change the thread::sleep time from config file
            std::thread::sleep(Duration::from_millis(100)) // Refresh color once per second (+ the time it takes to update)
        }
    }

    fn add_effect(&mut self, effect: Effect) {
        self.effects.push(effect)
    }

    fn add_effect_led(&mut self, led: CorsairLedLuid, effect: Effect) {
        self.key_effects.push((led, effect))
    }

    fn handle_msg(&mut self, connected: &mut bool, msg: CorsairMsg) {
        match msg {
            CorsairMsg::Connected => *connected = true,
            CorsairMsg::NotConnected => *connected = false,
            CorsairMsg::AddEffect(effect) => self.add_effect(*effect),
            CorsairMsg::AddEffectLed(led, effect) => self.add_effect_led(led, *effect),
            CorsairMsg::RemoveAllEffects => self.remove_all_effects(),
        }
    }

    fn remove_all_effects(&mut self) {
        self.effects = Vec::new();
        self.key_effects = Vec::new();
    }
}
