# plant_cli

A CLI for creating pretty animations of procedurally generated 'plants' using L-systems based on ['The Algorithmic Beauty of Plants'](http://algorithmicbotany.org/papers/#abop) by Aristid Lindenmayer and Przemysław Prusinkiewicz.

## Installation
Clone and build from source with Cargo.

## Usage
```
Usage: procedural_nannou [OPTIONS]

Options:
  -x, --rules_x <RULES_X>
          Sets the rules for X [default: F[+X][-X]FX]
  -f, --rules_f <RULES_F>
          Sets the rules for F
  -d, --delta <DELTA>
          Sets the delta angle [default: 25.7]
  -s, --speed <SPEED>
          Sets the animation speed [default: 3]
  -i, --iterations <ITERATIONS>
          Sets the number of iterations [default: 7]
  -b, --bg_color <BG_COLOR>
          Sets the background color (RGB format: R, G, B) [default: "(0, 0, 0)"]
  -t, --tree_color <TREE_COLOR>
          Sets the tree color (RGB format: R, G, B) [default: "(144, 238, 144)"]
  -c, --scaling_factor <SCALING_FACTOR>
          Sets the scaling factor for distance [default: 1.0]
  -h, --help
          Print help
  -V, --version
          Print version
```

## Examples
Args: -x "FF" -f "F[+F]F[-F][F]" -d 20 -i 6 -c 2.5
<img width="817" alt="image" src="https://user-images.githubusercontent.com/30433379/226842138-96a03f79-f204-4154-9a15-24ede4825da7.png">

Args: -x "F-[[X]+X]+F[+FX]-X" -f "FF" -d 22.5 -i 6 -t "(255,191,0)" -c 1.3
<img width="815" alt="image" src="https://user-images.githubusercontent.com/30433379/226844712-90767add-501c-4b30-80fd-0c9673b40f0a.png">

Args: -x "F[+X][-X]FX" -f "FF" -d 25.7 -i 7 -s 7 -t "(133,1,1)"
![image](https://user-images.githubusercontent.com/30433379/228568511-b49eb4d4-905c-4f74-b7d7-83523e53608e.png)

## Issues
Scale has to be adjusted until the whole tree shows as the height and width of the tree varies by the number of iterations and the rules. I tried to implement functionality that simulates the tree, measures the height/width and adjusts the scale accordingly but I failed because I suck at programming.
