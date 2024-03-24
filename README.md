# WIP
This project (technically works, for my setup) is not complete, therefore it will probably not work for what you want.

# Introduction
This tool allows you to set global hotkeys (remap keys), and control (corsair) keyboard lights (using iCUE). It only works on windows.

# How to use it
1. Download the iCUE SDK .dll and .lib
2. Set the CUE_SDK_LIB_FILES_PATH enviroment value to the directory where the SDK .lib is contained (only for compilation)
2. Add the folder containing the .dll to PATH (only for running the program)
3. Create a config.txt with your configuration in it
4. Compile and run the program `cargo run`

# generate config.txt
There *must* be at least 1 profile called "default"
```
profile default {

}
```

inside the profile, you can:
* Set hotkeys
* Set keyboard light effects
* Call a function

### Setting hotkeys
At the moment, you can only remap the keys F13,F14,F15...F24.

First, define a macro, for example a macro that does `Ctrl+J`:

```
macro my_ctrl_j_macro {
    press_key(Key::Ctrl)
    press_key(Key::J)
    release_key(Key::J)
    release_key(Key::Ctrl)
}
```

Then set the hotkey in a profile, for example the `default` profile that we created earlier.

```
profile default {
    set_hotkey(Key::F13, my_ctrl_j_macro)
}
```

This will press `ctrl+j` each time `F13` is pressed

### Keyboard light effects

There are 3 types of effects:
* static: Will turn the entire keyboard to this color
* wave: Will make a wave that travels along the keyboard
* ripple: Will make a circular wave from the center of the keyboard

The order in which the effects are declared is important. Setting a fully opaque static color on top of the other effects will hide them:

```
profile default {
    wave_effect(...)
    ripple_effect(...)
    wave_effect(...)
    ripple_effect(...)
    wave_effect(...)
    ripple_effect(...)
    wave_effect(...)
    ripple_effect(...)
    static_color((255, 0, 0, 255))
}
```

Will produce the same result as:

```
profile default {
    static_color((255, 0, 0, 255))
}
```

#### Static light
To add a static light, add it to a profile:

```
profile default {
    static_color((255,0,0,255))
}
```

This will make a fully opaque red color.

#### Wave
To add a wave effect, you must first create a color animation, like this:
```
color_animation red_and_blue {
    0.000 => (255,0,0,255)
    0.500 => (0,0,255,255)
    1.000 => (255,0,0,255)
}
```

At the left of the arrow, specify the timestamp (it must be between 0.0 (start of animation) and 1.0 (end of animation)). At the left, the color

Now we can add the wave effect to a profile using this color animation
```
profile default {
    wave_effect(red_and_blue, 1000, 5.0, 10.0, 0.0, false)
}
```

1. parameter: the color animation
2. parameter: time (in milliseconds) between one wave and the next
3. parameter: speed of the wave (in keys per second)
4. parameter: width of the wave (in keys)
5. parameter: angle of the wave (in degrees)
6. parameter: if true, it will make 2 waves (starting from the center, in opposite directions), if not, it will make just 1 wave starting from the left.

#### Ripple

To add a ripple effect, you must first create a color animation, like this:
```
color_animation red_and_blue {
    0.000 => (255,0,0,255)
    0.500 => (0,0,255,255)
    1.000 => (255,0,0,255)
}
```

At the left of the arrow, specify the timestamp (it must be between 0.0 (start of animation) and 1.0 (end of animation)). At the left, the color.

Now we can add the ripple effect to a profile using this color animation
```
profile default {
    ripple_effect(red_and_blue, 2500, 5.0, 10.0)
}
```

1. parameter: the color animation
2. parameter: time (in milliseconds) between one ripple and the next
3. parameter: speed of the ripple (in keys per second)
4. parameter: width of the ripple (in keys)

### Functions
To declare a function:
```
fn my_function {

}
```

Inside, you can set hotkeys and light effects, just like profiles.

Functions are useful if you want your light effects (or hotkeys) to be constant accross your profiles, for example:
```
fn my_light_effects {
    static_color((255,0,0,255))
    wave_effect(red_and_blue, 1000, 5.0, 10.0, 0.0, false)
    ripple_effect(red_and_blue, 2500, 5.0, 10.0)
}

profile default {
    my_light_effects()
}

profile another_profile {
    my_light_effects()
}
```

They can also be used to maintain constant only part of the light effects (or keys):

```
fn my_base_color {
    static_color((255,0,0,255))
}

profile default {
    my_light_effects()
    wave_effect(red_and_blue, 1000, 5.0, 10.0, 0.0, false)
}

profile another_profile {
    my_light_effects()
    ripple_effect(red_and_blue, 2500, 5.0, 10.0)
}
```

## Switching profiles
To switch profile, you must declare a macro and set a hotkey:

```
macro switch_to_default {
    switch_profile(default)
}

macro switch_to_other_profile {
    switch_profile(other_profile)
}

profile default {
    set_hotkey(Key::F13, switch_to_other_profile)
}

profile other_profile {
    set_hotkey(Key::F13, switch_to_default)
}
```
