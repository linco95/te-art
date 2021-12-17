# te-art
Rust reservation art generator for #TEArt2021

```
╔╦╗╔═╗  ╔═╗╦═╗╔╦╗
 ║ ║╣   ╠═╣╠╦╝ ║ 
 ╩ ╚═╝  ╩ ╩╩╚═ ╩ 
 ```
Developed by: Andreas Kjellqvist.
Special thanks to Göran Hällgren, Fredrik Björeman for help with XML importer
****************************************************
Welcome to my TE Art project!
The first time you run this program it will crash as you need to fill out the config file!
To run the tool just update the config file (teart_cfg) and run it with path to the image you want to create. 
After it is done converting the image it will provide a preview in output folder. 
If the preview looks good you can use the XML Importer to import the reservations into TimeEdit. 
Just select the xml payload file in output folder as import file.

Example:
1. `./teart`
2. Update the generated config
3. `./teart <path_to_image>`
****************************************************
# The config file contains of the following parameters:
### login_name: String, 
Username of the user importing the reservations
### size: u32,              
The size of the final image, works best with increments of 2 (2, 4, 8, 16, 32, 64, 128) (recommended max 128 as that's the most that fits in 24 hours)
### start_datetime: String,
The starting datetime for the reservations. Recommended to be 00:00. This will be converted to local timezone of the importing user
### reservation_mode: String,
Reservation mode for the import
### org: String,
Organization to do the reservations for
### canvas_object: (String, String),
This object (type extid, object extid) is added to all reservations. This is what is selected to view the painting. You could have multiple drawings in the same TE database by having each drawing on a canvas object, and swich which canvas object is selected
### color_objects: Vec<(String, String)>,
List of the color objects (type extid, object extid). These are expected to be colored in TimeEdit, in the same order as the list of solid colors, and with the last object being uncolored (for grey color)
### auth_server: String,
For xml_importer this should be timeedit
****************************************************
