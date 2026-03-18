> Generated from `nix run github:bugeats/useful-tui-chars`

# Useful TUI Characters

Escape sequences use Rust `\u{XXXX}` string format.
Character names are official designations from the [Unicode Character Database](https://www.unicode.org/ucd/).

- [Box Drawing](#box-drawing-128) (128)
- [Block Elements](#block-elements-32) (32)
- [Arrows](#arrows-112) (112)
- [Geometric Shapes](#geometric-shapes-96) (96)
- [Mathematical Operators](#mathematical-operators-256) (256)
- [Greek and Coptic](#greek-and-coptic-144) (144)
- [Letterlike Symbols](#letterlike-symbols-80) (80)
- [Braille Patterns](#braille-patterns-256) (256)
- [Miscellaneous Technical](#miscellaneous-technical-256) (256)
- [Dingbats](#dingbats-192) (192)
- [Enclosed Alphanumerics](#enclosed-alphanumerics-160) (160)
- [Miscellaneous Symbols](#miscellaneous-symbols-256) (256)
- [Miscellaneous Symbols and Arrows](#miscellaneous-symbols-and-arrows-256) (256)
- [Symbols for Legacy Computing](#symbols-for-legacy-computing-256) (256)
- [Latin-1 Supplement](#latin-1-supplement-96) (96)
- [CJK Compatibility](#cjk-compatibility-111) (111)

## Box Drawing (128)

| Char | Escape     | Name                                                  |
|------|------------|-------------------------------------------------------|
| ─    | `\u{2500}` | BOX DRAWINGS LIGHT HORIZONTAL                         |
| ━    | `\u{2501}` | BOX DRAWINGS HEAVY HORIZONTAL                         |
| │    | `\u{2502}` | BOX DRAWINGS LIGHT VERTICAL                           |
| ┃    | `\u{2503}` | BOX DRAWINGS HEAVY VERTICAL                           |
| ┄    | `\u{2504}` | BOX DRAWINGS LIGHT TRIPLE DASH HORIZONTAL             |
| ┅    | `\u{2505}` | BOX DRAWINGS HEAVY TRIPLE DASH HORIZONTAL             |
| ┆    | `\u{2506}` | BOX DRAWINGS LIGHT TRIPLE DASH VERTICAL               |
| ┇    | `\u{2507}` | BOX DRAWINGS HEAVY TRIPLE DASH VERTICAL               |
| ┈    | `\u{2508}` | BOX DRAWINGS LIGHT QUADRUPLE DASH HORIZONTAL          |
| ┉    | `\u{2509}` | BOX DRAWINGS HEAVY QUADRUPLE DASH HORIZONTAL          |
| ┊    | `\u{250A}` | BOX DRAWINGS LIGHT QUADRUPLE DASH VERTICAL            |
| ┋    | `\u{250B}` | BOX DRAWINGS HEAVY QUADRUPLE DASH VERTICAL            |
| ┌    | `\u{250C}` | BOX DRAWINGS LIGHT DOWN AND RIGHT                     |
| ┍    | `\u{250D}` | BOX DRAWINGS DOWN LIGHT AND RIGHT HEAVY               |
| ┎    | `\u{250E}` | BOX DRAWINGS DOWN HEAVY AND RIGHT LIGHT               |
| ┏    | `\u{250F}` | BOX DRAWINGS HEAVY DOWN AND RIGHT                     |
| ┐    | `\u{2510}` | BOX DRAWINGS LIGHT DOWN AND LEFT                      |
| ┑    | `\u{2511}` | BOX DRAWINGS DOWN LIGHT AND LEFT HEAVY                |
| ┒    | `\u{2512}` | BOX DRAWINGS DOWN HEAVY AND LEFT LIGHT                |
| ┓    | `\u{2513}` | BOX DRAWINGS HEAVY DOWN AND LEFT                      |
| └    | `\u{2514}` | BOX DRAWINGS LIGHT UP AND RIGHT                       |
| ┕    | `\u{2515}` | BOX DRAWINGS UP LIGHT AND RIGHT HEAVY                 |
| ┖    | `\u{2516}` | BOX DRAWINGS UP HEAVY AND RIGHT LIGHT                 |
| ┗    | `\u{2517}` | BOX DRAWINGS HEAVY UP AND RIGHT                       |
| ┘    | `\u{2518}` | BOX DRAWINGS LIGHT UP AND LEFT                        |
| ┙    | `\u{2519}` | BOX DRAWINGS UP LIGHT AND LEFT HEAVY                  |
| ┚    | `\u{251A}` | BOX DRAWINGS UP HEAVY AND LEFT LIGHT                  |
| ┛    | `\u{251B}` | BOX DRAWINGS HEAVY UP AND LEFT                        |
| ├    | `\u{251C}` | BOX DRAWINGS LIGHT VERTICAL AND RIGHT                 |
| ┝    | `\u{251D}` | BOX DRAWINGS VERTICAL LIGHT AND RIGHT HEAVY           |
| ┞    | `\u{251E}` | BOX DRAWINGS UP HEAVY AND RIGHT DOWN LIGHT            |
| ┟    | `\u{251F}` | BOX DRAWINGS DOWN HEAVY AND RIGHT UP LIGHT            |
| ┠    | `\u{2520}` | BOX DRAWINGS VERTICAL HEAVY AND RIGHT LIGHT           |
| ┡    | `\u{2521}` | BOX DRAWINGS DOWN LIGHT AND RIGHT UP HEAVY            |
| ┢    | `\u{2522}` | BOX DRAWINGS UP LIGHT AND RIGHT DOWN HEAVY            |
| ┣    | `\u{2523}` | BOX DRAWINGS HEAVY VERTICAL AND RIGHT                 |
| ┤    | `\u{2524}` | BOX DRAWINGS LIGHT VERTICAL AND LEFT                  |
| ┥    | `\u{2525}` | BOX DRAWINGS VERTICAL LIGHT AND LEFT HEAVY            |
| ┦    | `\u{2526}` | BOX DRAWINGS UP HEAVY AND LEFT DOWN LIGHT             |
| ┧    | `\u{2527}` | BOX DRAWINGS DOWN HEAVY AND LEFT UP LIGHT             |
| ┨    | `\u{2528}` | BOX DRAWINGS VERTICAL HEAVY AND LEFT LIGHT            |
| ┩    | `\u{2529}` | BOX DRAWINGS DOWN LIGHT AND LEFT UP HEAVY             |
| ┪    | `\u{252A}` | BOX DRAWINGS UP LIGHT AND LEFT DOWN HEAVY             |
| ┫    | `\u{252B}` | BOX DRAWINGS HEAVY VERTICAL AND LEFT                  |
| ┬    | `\u{252C}` | BOX DRAWINGS LIGHT DOWN AND HORIZONTAL                |
| ┭    | `\u{252D}` | BOX DRAWINGS LEFT HEAVY AND RIGHT DOWN LIGHT          |
| ┮    | `\u{252E}` | BOX DRAWINGS RIGHT HEAVY AND LEFT DOWN LIGHT          |
| ┯    | `\u{252F}` | BOX DRAWINGS DOWN LIGHT AND HORIZONTAL HEAVY          |
| ┰    | `\u{2530}` | BOX DRAWINGS DOWN HEAVY AND HORIZONTAL LIGHT          |
| ┱    | `\u{2531}` | BOX DRAWINGS RIGHT LIGHT AND LEFT DOWN HEAVY          |
| ┲    | `\u{2532}` | BOX DRAWINGS LEFT LIGHT AND RIGHT DOWN HEAVY          |
| ┳    | `\u{2533}` | BOX DRAWINGS HEAVY DOWN AND HORIZONTAL                |
| ┴    | `\u{2534}` | BOX DRAWINGS LIGHT UP AND HORIZONTAL                  |
| ┵    | `\u{2535}` | BOX DRAWINGS LEFT HEAVY AND RIGHT UP LIGHT            |
| ┶    | `\u{2536}` | BOX DRAWINGS RIGHT HEAVY AND LEFT UP LIGHT            |
| ┷    | `\u{2537}` | BOX DRAWINGS UP LIGHT AND HORIZONTAL HEAVY            |
| ┸    | `\u{2538}` | BOX DRAWINGS UP HEAVY AND HORIZONTAL LIGHT            |
| ┹    | `\u{2539}` | BOX DRAWINGS RIGHT LIGHT AND LEFT UP HEAVY            |
| ┺    | `\u{253A}` | BOX DRAWINGS LEFT LIGHT AND RIGHT UP HEAVY            |
| ┻    | `\u{253B}` | BOX DRAWINGS HEAVY UP AND HORIZONTAL                  |
| ┼    | `\u{253C}` | BOX DRAWINGS LIGHT VERTICAL AND HORIZONTAL            |
| ┽    | `\u{253D}` | BOX DRAWINGS LEFT HEAVY AND RIGHT VERTICAL LIGHT      |
| ┾    | `\u{253E}` | BOX DRAWINGS RIGHT HEAVY AND LEFT VERTICAL LIGHT      |
| ┿    | `\u{253F}` | BOX DRAWINGS VERTICAL LIGHT AND HORIZONTAL HEAVY      |
| ╀    | `\u{2540}` | BOX DRAWINGS UP HEAVY AND DOWN HORIZONTAL LIGHT       |
| ╁    | `\u{2541}` | BOX DRAWINGS DOWN HEAVY AND UP HORIZONTAL LIGHT       |
| ╂    | `\u{2542}` | BOX DRAWINGS VERTICAL HEAVY AND HORIZONTAL LIGHT      |
| ╃    | `\u{2543}` | BOX DRAWINGS LEFT UP HEAVY AND RIGHT DOWN LIGHT       |
| ╄    | `\u{2544}` | BOX DRAWINGS RIGHT UP HEAVY AND LEFT DOWN LIGHT       |
| ╅    | `\u{2545}` | BOX DRAWINGS LEFT DOWN HEAVY AND RIGHT UP LIGHT       |
| ╆    | `\u{2546}` | BOX DRAWINGS RIGHT DOWN HEAVY AND LEFT UP LIGHT       |
| ╇    | `\u{2547}` | BOX DRAWINGS DOWN LIGHT AND UP HORIZONTAL HEAVY       |
| ╈    | `\u{2548}` | BOX DRAWINGS UP LIGHT AND DOWN HORIZONTAL HEAVY       |
| ╉    | `\u{2549}` | BOX DRAWINGS RIGHT LIGHT AND LEFT VERTICAL HEAVY      |
| ╊    | `\u{254A}` | BOX DRAWINGS LEFT LIGHT AND RIGHT VERTICAL HEAVY      |
| ╋    | `\u{254B}` | BOX DRAWINGS HEAVY VERTICAL AND HORIZONTAL            |
| ╌    | `\u{254C}` | BOX DRAWINGS LIGHT DOUBLE DASH HORIZONTAL             |
| ╍    | `\u{254D}` | BOX DRAWINGS HEAVY DOUBLE DASH HORIZONTAL             |
| ╎    | `\u{254E}` | BOX DRAWINGS LIGHT DOUBLE DASH VERTICAL               |
| ╏    | `\u{254F}` | BOX DRAWINGS HEAVY DOUBLE DASH VERTICAL               |
| ═    | `\u{2550}` | BOX DRAWINGS DOUBLE HORIZONTAL                        |
| ║    | `\u{2551}` | BOX DRAWINGS DOUBLE VERTICAL                          |
| ╒    | `\u{2552}` | BOX DRAWINGS DOWN SINGLE AND RIGHT DOUBLE             |
| ╓    | `\u{2553}` | BOX DRAWINGS DOWN DOUBLE AND RIGHT SINGLE             |
| ╔    | `\u{2554}` | BOX DRAWINGS DOUBLE DOWN AND RIGHT                    |
| ╕    | `\u{2555}` | BOX DRAWINGS DOWN SINGLE AND LEFT DOUBLE              |
| ╖    | `\u{2556}` | BOX DRAWINGS DOWN DOUBLE AND LEFT SINGLE              |
| ╗    | `\u{2557}` | BOX DRAWINGS DOUBLE DOWN AND LEFT                     |
| ╘    | `\u{2558}` | BOX DRAWINGS UP SINGLE AND RIGHT DOUBLE               |
| ╙    | `\u{2559}` | BOX DRAWINGS UP DOUBLE AND RIGHT SINGLE               |
| ╚    | `\u{255A}` | BOX DRAWINGS DOUBLE UP AND RIGHT                      |
| ╛    | `\u{255B}` | BOX DRAWINGS UP SINGLE AND LEFT DOUBLE                |
| ╜    | `\u{255C}` | BOX DRAWINGS UP DOUBLE AND LEFT SINGLE                |
| ╝    | `\u{255D}` | BOX DRAWINGS DOUBLE UP AND LEFT                       |
| ╞    | `\u{255E}` | BOX DRAWINGS VERTICAL SINGLE AND RIGHT DOUBLE         |
| ╟    | `\u{255F}` | BOX DRAWINGS VERTICAL DOUBLE AND RIGHT SINGLE         |
| ╠    | `\u{2560}` | BOX DRAWINGS DOUBLE VERTICAL AND RIGHT                |
| ╡    | `\u{2561}` | BOX DRAWINGS VERTICAL SINGLE AND LEFT DOUBLE          |
| ╢    | `\u{2562}` | BOX DRAWINGS VERTICAL DOUBLE AND LEFT SINGLE          |
| ╣    | `\u{2563}` | BOX DRAWINGS DOUBLE VERTICAL AND LEFT                 |
| ╤    | `\u{2564}` | BOX DRAWINGS DOWN SINGLE AND HORIZONTAL DOUBLE        |
| ╥    | `\u{2565}` | BOX DRAWINGS DOWN DOUBLE AND HORIZONTAL SINGLE        |
| ╦    | `\u{2566}` | BOX DRAWINGS DOUBLE DOWN AND HORIZONTAL               |
| ╧    | `\u{2567}` | BOX DRAWINGS UP SINGLE AND HORIZONTAL DOUBLE          |
| ╨    | `\u{2568}` | BOX DRAWINGS UP DOUBLE AND HORIZONTAL SINGLE          |
| ╩    | `\u{2569}` | BOX DRAWINGS DOUBLE UP AND HORIZONTAL                 |
| ╪    | `\u{256A}` | BOX DRAWINGS VERTICAL SINGLE AND HORIZONTAL DOUBLE    |
| ╫    | `\u{256B}` | BOX DRAWINGS VERTICAL DOUBLE AND HORIZONTAL SINGLE    |
| ╬    | `\u{256C}` | BOX DRAWINGS DOUBLE VERTICAL AND HORIZONTAL           |
| ╭    | `\u{256D}` | BOX DRAWINGS LIGHT ARC DOWN AND RIGHT                 |
| ╮    | `\u{256E}` | BOX DRAWINGS LIGHT ARC DOWN AND LEFT                  |
| ╯    | `\u{256F}` | BOX DRAWINGS LIGHT ARC UP AND LEFT                    |
| ╰    | `\u{2570}` | BOX DRAWINGS LIGHT ARC UP AND RIGHT                   |
| ╱    | `\u{2571}` | BOX DRAWINGS LIGHT DIAGONAL UPPER RIGHT TO LOWER LEFT |
| ╲    | `\u{2572}` | BOX DRAWINGS LIGHT DIAGONAL UPPER LEFT TO LOWER RIGHT |
| ╳    | `\u{2573}` | BOX DRAWINGS LIGHT DIAGONAL CROSS                     |
| ╴    | `\u{2574}` | BOX DRAWINGS LIGHT LEFT                               |
| ╵    | `\u{2575}` | BOX DRAWINGS LIGHT UP                                 |
| ╶    | `\u{2576}` | BOX DRAWINGS LIGHT RIGHT                              |
| ╷    | `\u{2577}` | BOX DRAWINGS LIGHT DOWN                               |
| ╸    | `\u{2578}` | BOX DRAWINGS HEAVY LEFT                               |
| ╹    | `\u{2579}` | BOX DRAWINGS HEAVY UP                                 |
| ╺    | `\u{257A}` | BOX DRAWINGS HEAVY RIGHT                              |
| ╻    | `\u{257B}` | BOX DRAWINGS HEAVY DOWN                               |
| ╼    | `\u{257C}` | BOX DRAWINGS LIGHT LEFT AND HEAVY RIGHT               |
| ╽    | `\u{257D}` | BOX DRAWINGS LIGHT UP AND HEAVY DOWN                  |
| ╾    | `\u{257E}` | BOX DRAWINGS HEAVY LEFT AND LIGHT RIGHT               |
| ╿    | `\u{257F}` | BOX DRAWINGS HEAVY UP AND LIGHT DOWN                  |

## Block Elements (32)

| Char | Escape     | Name                                                |
|------|------------|-----------------------------------------------------|
| ▀    | `\u{2580}` | UPPER HALF BLOCK                                    |
| ▁    | `\u{2581}` | LOWER ONE EIGHTH BLOCK                              |
| ▂    | `\u{2582}` | LOWER ONE QUARTER BLOCK                             |
| ▃    | `\u{2583}` | LOWER THREE EIGHTHS BLOCK                           |
| ▄    | `\u{2584}` | LOWER HALF BLOCK                                    |
| ▅    | `\u{2585}` | LOWER FIVE EIGHTHS BLOCK                            |
| ▆    | `\u{2586}` | LOWER THREE QUARTERS BLOCK                          |
| ▇    | `\u{2587}` | LOWER SEVEN EIGHTHS BLOCK                           |
| █    | `\u{2588}` | FULL BLOCK                                          |
| ▉    | `\u{2589}` | LEFT SEVEN EIGHTHS BLOCK                            |
| ▊    | `\u{258A}` | LEFT THREE QUARTERS BLOCK                           |
| ▋    | `\u{258B}` | LEFT FIVE EIGHTHS BLOCK                             |
| ▌    | `\u{258C}` | LEFT HALF BLOCK                                     |
| ▍    | `\u{258D}` | LEFT THREE EIGHTHS BLOCK                            |
| ▎    | `\u{258E}` | LEFT ONE QUARTER BLOCK                              |
| ▏    | `\u{258F}` | LEFT ONE EIGHTH BLOCK                               |
| ▐    | `\u{2590}` | RIGHT HALF BLOCK                                    |
| ░    | `\u{2591}` | LIGHT SHADE                                         |
| ▒    | `\u{2592}` | MEDIUM SHADE                                        |
| ▓    | `\u{2593}` | DARK SHADE                                          |
| ▔    | `\u{2594}` | UPPER ONE EIGHTH BLOCK                              |
| ▕    | `\u{2595}` | RIGHT ONE EIGHTH BLOCK                              |
| ▖    | `\u{2596}` | QUADRANT LOWER LEFT                                 |
| ▗    | `\u{2597}` | QUADRANT LOWER RIGHT                                |
| ▘    | `\u{2598}` | QUADRANT UPPER LEFT                                 |
| ▙    | `\u{2599}` | QUADRANT UPPER LEFT AND LOWER LEFT AND LOWER RIGHT  |
| ▚    | `\u{259A}` | QUADRANT UPPER LEFT AND LOWER RIGHT                 |
| ▛    | `\u{259B}` | QUADRANT UPPER LEFT AND UPPER RIGHT AND LOWER LEFT  |
| ▜    | `\u{259C}` | QUADRANT UPPER LEFT AND UPPER RIGHT AND LOWER RIGHT |
| ▝    | `\u{259D}` | QUADRANT UPPER RIGHT                                |
| ▞    | `\u{259E}` | QUADRANT UPPER RIGHT AND LOWER LEFT                 |
| ▟    | `\u{259F}` | QUADRANT UPPER RIGHT AND LOWER LEFT AND LOWER RIGHT |

## Arrows (112)

| Char | Escape     | Name                                                |
|------|------------|-----------------------------------------------------|
| ←    | `\u{2190}` | LEFTWARDS ARROW                                     |
| ↑    | `\u{2191}` | UPWARDS ARROW                                       |
| →    | `\u{2192}` | RIGHTWARDS ARROW                                    |
| ↓    | `\u{2193}` | DOWNWARDS ARROW                                     |
| ↔    | `\u{2194}` | LEFT RIGHT ARROW                                    |
| ↕    | `\u{2195}` | UP DOWN ARROW                                       |
| ↖    | `\u{2196}` | NORTH WEST ARROW                                    |
| ↗    | `\u{2197}` | NORTH EAST ARROW                                    |
| ↘    | `\u{2198}` | SOUTH EAST ARROW                                    |
| ↙    | `\u{2199}` | SOUTH WEST ARROW                                    |
| ↚    | `\u{219A}` | LEFTWARDS ARROW WITH STROKE                         |
| ↛    | `\u{219B}` | RIGHTWARDS ARROW WITH STROKE                        |
| ↜    | `\u{219C}` | LEFTWARDS WAVE ARROW                                |
| ↝    | `\u{219D}` | RIGHTWARDS WAVE ARROW                               |
| ↞    | `\u{219E}` | LEFTWARDS TWO HEADED ARROW                          |
| ↟    | `\u{219F}` | UPWARDS TWO HEADED ARROW                            |
| ↠    | `\u{21A0}` | RIGHTWARDS TWO HEADED ARROW                         |
| ↡    | `\u{21A1}` | DOWNWARDS TWO HEADED ARROW                          |
| ↢    | `\u{21A2}` | LEFTWARDS ARROW WITH TAIL                           |
| ↣    | `\u{21A3}` | RIGHTWARDS ARROW WITH TAIL                          |
| ↤    | `\u{21A4}` | LEFTWARDS ARROW FROM BAR                            |
| ↥    | `\u{21A5}` | UPWARDS ARROW FROM BAR                              |
| ↦    | `\u{21A6}` | RIGHTWARDS ARROW FROM BAR                           |
| ↧    | `\u{21A7}` | DOWNWARDS ARROW FROM BAR                            |
| ↨    | `\u{21A8}` | UP DOWN ARROW WITH BASE                             |
| ↩    | `\u{21A9}` | LEFTWARDS ARROW WITH HOOK                           |
| ↪    | `\u{21AA}` | RIGHTWARDS ARROW WITH HOOK                          |
| ↫    | `\u{21AB}` | LEFTWARDS ARROW WITH LOOP                           |
| ↬    | `\u{21AC}` | RIGHTWARDS ARROW WITH LOOP                          |
| ↭    | `\u{21AD}` | LEFT RIGHT WAVE ARROW                               |
| ↮    | `\u{21AE}` | LEFT RIGHT ARROW WITH STROKE                        |
| ↯    | `\u{21AF}` | DOWNWARDS ZIGZAG ARROW                              |
| ↰    | `\u{21B0}` | UPWARDS ARROW WITH TIP LEFTWARDS                    |
| ↱    | `\u{21B1}` | UPWARDS ARROW WITH TIP RIGHTWARDS                   |
| ↲    | `\u{21B2}` | DOWNWARDS ARROW WITH TIP LEFTWARDS                  |
| ↳    | `\u{21B3}` | DOWNWARDS ARROW WITH TIP RIGHTWARDS                 |
| ↴    | `\u{21B4}` | RIGHTWARDS ARROW WITH CORNER DOWNWARDS              |
| ↵    | `\u{21B5}` | DOWNWARDS ARROW WITH CORNER LEFTWARDS               |
| ↶    | `\u{21B6}` | ANTICLOCKWISE TOP SEMICIRCLE ARROW                  |
| ↷    | `\u{21B7}` | CLOCKWISE TOP SEMICIRCLE ARROW                      |
| ↸    | `\u{21B8}` | NORTH WEST ARROW TO LONG BAR                        |
| ↹    | `\u{21B9}` | LEFTWARDS ARROW TO BAR OVER RIGHTWARDS ARROW TO BAR |
| ↺    | `\u{21BA}` | ANTICLOCKWISE OPEN CIRCLE ARROW                     |
| ↻    | `\u{21BB}` | CLOCKWISE OPEN CIRCLE ARROW                         |
| ↼    | `\u{21BC}` | LEFTWARDS HARPOON WITH BARB UPWARDS                 |
| ↽    | `\u{21BD}` | LEFTWARDS HARPOON WITH BARB DOWNWARDS               |
| ↾    | `\u{21BE}` | UPWARDS HARPOON WITH BARB RIGHTWARDS                |
| ↿    | `\u{21BF}` | UPWARDS HARPOON WITH BARB LEFTWARDS                 |
| ⇀    | `\u{21C0}` | RIGHTWARDS HARPOON WITH BARB UPWARDS                |
| ⇁    | `\u{21C1}` | RIGHTWARDS HARPOON WITH BARB DOWNWARDS              |
| ⇂    | `\u{21C2}` | DOWNWARDS HARPOON WITH BARB RIGHTWARDS              |
| ⇃    | `\u{21C3}` | DOWNWARDS HARPOON WITH BARB LEFTWARDS               |
| ⇄    | `\u{21C4}` | RIGHTWARDS ARROW OVER LEFTWARDS ARROW               |
| ⇅    | `\u{21C5}` | UPWARDS ARROW LEFTWARDS OF DOWNWARDS ARROW          |
| ⇆    | `\u{21C6}` | LEFTWARDS ARROW OVER RIGHTWARDS ARROW               |
| ⇇    | `\u{21C7}` | LEFTWARDS PAIRED ARROWS                             |
| ⇈    | `\u{21C8}` | UPWARDS PAIRED ARROWS                               |
| ⇉    | `\u{21C9}` | RIGHTWARDS PAIRED ARROWS                            |
| ⇊    | `\u{21CA}` | DOWNWARDS PAIRED ARROWS                             |
| ⇋    | `\u{21CB}` | LEFTWARDS HARPOON OVER RIGHTWARDS HARPOON           |
| ⇌    | `\u{21CC}` | RIGHTWARDS HARPOON OVER LEFTWARDS HARPOON           |
| ⇍    | `\u{21CD}` | LEFTWARDS DOUBLE ARROW WITH STROKE                  |
| ⇎    | `\u{21CE}` | LEFT RIGHT DOUBLE ARROW WITH STROKE                 |
| ⇏    | `\u{21CF}` | RIGHTWARDS DOUBLE ARROW WITH STROKE                 |
| ⇐    | `\u{21D0}` | LEFTWARDS DOUBLE ARROW                              |
| ⇑    | `\u{21D1}` | UPWARDS DOUBLE ARROW                                |
| ⇒    | `\u{21D2}` | RIGHTWARDS DOUBLE ARROW                             |
| ⇓    | `\u{21D3}` | DOWNWARDS DOUBLE ARROW                              |
| ⇔    | `\u{21D4}` | LEFT RIGHT DOUBLE ARROW                             |
| ⇕    | `\u{21D5}` | UP DOWN DOUBLE ARROW                                |
| ⇖    | `\u{21D6}` | NORTH WEST DOUBLE ARROW                             |
| ⇗    | `\u{21D7}` | NORTH EAST DOUBLE ARROW                             |
| ⇘    | `\u{21D8}` | SOUTH EAST DOUBLE ARROW                             |
| ⇙    | `\u{21D9}` | SOUTH WEST DOUBLE ARROW                             |
| ⇚    | `\u{21DA}` | LEFTWARDS TRIPLE ARROW                              |
| ⇛    | `\u{21DB}` | RIGHTWARDS TRIPLE ARROW                             |
| ⇜    | `\u{21DC}` | LEFTWARDS SQUIGGLE ARROW                            |
| ⇝    | `\u{21DD}` | RIGHTWARDS SQUIGGLE ARROW                           |
| ⇞    | `\u{21DE}` | UPWARDS ARROW WITH DOUBLE STROKE                    |
| ⇟    | `\u{21DF}` | DOWNWARDS ARROW WITH DOUBLE STROKE                  |
| ⇠    | `\u{21E0}` | LEFTWARDS DASHED ARROW                              |
| ⇡    | `\u{21E1}` | UPWARDS DASHED ARROW                                |
| ⇢    | `\u{21E2}` | RIGHTWARDS DASHED ARROW                             |
| ⇣    | `\u{21E3}` | DOWNWARDS DASHED ARROW                              |
| ⇤    | `\u{21E4}` | LEFTWARDS ARROW TO BAR                              |
| ⇥    | `\u{21E5}` | RIGHTWARDS ARROW TO BAR                             |
| ⇦    | `\u{21E6}` | LEFTWARDS WHITE ARROW                               |
| ⇧    | `\u{21E7}` | UPWARDS WHITE ARROW                                 |
| ⇨    | `\u{21E8}` | RIGHTWARDS WHITE ARROW                              |
| ⇩    | `\u{21E9}` | DOWNWARDS WHITE ARROW                               |
| ⇪    | `\u{21EA}` | UPWARDS WHITE ARROW FROM BAR                        |
| ⇫    | `\u{21EB}` | UPWARDS WHITE ARROW ON PEDESTAL                     |
| ⇬    | `\u{21EC}` | UPWARDS WHITE ARROW ON PEDESTAL WITH HORIZONTAL BAR |
| ⇭    | `\u{21ED}` | UPWARDS WHITE ARROW ON PEDESTAL WITH VERTICAL BAR   |
| ⇮    | `\u{21EE}` | UPWARDS WHITE DOUBLE ARROW                          |
| ⇯    | `\u{21EF}` | UPWARDS WHITE DOUBLE ARROW ON PEDESTAL              |
| ⇰    | `\u{21F0}` | RIGHTWARDS WHITE ARROW FROM WALL                    |
| ⇱    | `\u{21F1}` | NORTH WEST ARROW TO CORNER                          |
| ⇲    | `\u{21F2}` | SOUTH EAST ARROW TO CORNER                          |
| ⇳    | `\u{21F3}` | UP DOWN WHITE ARROW                                 |
| ⇴    | `\u{21F4}` | RIGHT ARROW WITH SMALL CIRCLE                       |
| ⇵    | `\u{21F5}` | DOWNWARDS ARROW LEFTWARDS OF UPWARDS ARROW          |
| ⇶    | `\u{21F6}` | THREE RIGHTWARDS ARROWS                             |
| ⇷    | `\u{21F7}` | LEFTWARDS ARROW WITH VERTICAL STROKE                |
| ⇸    | `\u{21F8}` | RIGHTWARDS ARROW WITH VERTICAL STROKE               |
| ⇹    | `\u{21F9}` | LEFT RIGHT ARROW WITH VERTICAL STROKE               |
| ⇺    | `\u{21FA}` | LEFTWARDS ARROW WITH DOUBLE VERTICAL STROKE         |
| ⇻    | `\u{21FB}` | RIGHTWARDS ARROW WITH DOUBLE VERTICAL STROKE        |
| ⇼    | `\u{21FC}` | LEFT RIGHT ARROW WITH DOUBLE VERTICAL STROKE        |
| ⇽    | `\u{21FD}` | LEFTWARDS OPEN-HEADED ARROW                         |
| ⇾    | `\u{21FE}` | RIGHTWARDS OPEN-HEADED ARROW                        |
| ⇿    | `\u{21FF}` | LEFT RIGHT OPEN-HEADED ARROW                        |

## Geometric Shapes (96)

| Char | Escape     | Name                                          |
|------|------------|-----------------------------------------------|
| ■    | `\u{25A0}` | BLACK SQUARE                                  |
| □    | `\u{25A1}` | WHITE SQUARE                                  |
| ▢    | `\u{25A2}` | WHITE SQUARE WITH ROUNDED CORNERS             |
| ▣    | `\u{25A3}` | WHITE SQUARE CONTAINING BLACK SMALL SQUARE    |
| ▤    | `\u{25A4}` | SQUARE WITH HORIZONTAL FILL                   |
| ▥    | `\u{25A5}` | SQUARE WITH VERTICAL FILL                     |
| ▦    | `\u{25A6}` | SQUARE WITH ORTHOGONAL CROSSHATCH FILL        |
| ▧    | `\u{25A7}` | SQUARE WITH UPPER LEFT TO LOWER RIGHT FILL    |
| ▨    | `\u{25A8}` | SQUARE WITH UPPER RIGHT TO LOWER LEFT FILL    |
| ▩    | `\u{25A9}` | SQUARE WITH DIAGONAL CROSSHATCH FILL          |
| ▪    | `\u{25AA}` | BLACK SMALL SQUARE                            |
| ▫    | `\u{25AB}` | WHITE SMALL SQUARE                            |
| ▬    | `\u{25AC}` | BLACK RECTANGLE                               |
| ▭    | `\u{25AD}` | WHITE RECTANGLE                               |
| ▮    | `\u{25AE}` | BLACK VERTICAL RECTANGLE                      |
| ▯    | `\u{25AF}` | WHITE VERTICAL RECTANGLE                      |
| ▰    | `\u{25B0}` | BLACK PARALLELOGRAM                           |
| ▱    | `\u{25B1}` | WHITE PARALLELOGRAM                           |
| ▲    | `\u{25B2}` | BLACK UP-POINTING TRIANGLE                    |
| △    | `\u{25B3}` | WHITE UP-POINTING TRIANGLE                    |
| ▴    | `\u{25B4}` | BLACK UP-POINTING SMALL TRIANGLE              |
| ▵    | `\u{25B5}` | WHITE UP-POINTING SMALL TRIANGLE              |
| ▶    | `\u{25B6}` | BLACK RIGHT-POINTING TRIANGLE                 |
| ▷    | `\u{25B7}` | WHITE RIGHT-POINTING TRIANGLE                 |
| ▸    | `\u{25B8}` | BLACK RIGHT-POINTING SMALL TRIANGLE           |
| ▹    | `\u{25B9}` | WHITE RIGHT-POINTING SMALL TRIANGLE           |
| ►    | `\u{25BA}` | BLACK RIGHT-POINTING POINTER                  |
| ▻    | `\u{25BB}` | WHITE RIGHT-POINTING POINTER                  |
| ▼    | `\u{25BC}` | BLACK DOWN-POINTING TRIANGLE                  |
| ▽    | `\u{25BD}` | WHITE DOWN-POINTING TRIANGLE                  |
| ▾    | `\u{25BE}` | BLACK DOWN-POINTING SMALL TRIANGLE            |
| ▿    | `\u{25BF}` | WHITE DOWN-POINTING SMALL TRIANGLE            |
| ◀    | `\u{25C0}` | BLACK LEFT-POINTING TRIANGLE                  |
| ◁    | `\u{25C1}` | WHITE LEFT-POINTING TRIANGLE                  |
| ◂    | `\u{25C2}` | BLACK LEFT-POINTING SMALL TRIANGLE            |
| ◃    | `\u{25C3}` | WHITE LEFT-POINTING SMALL TRIANGLE            |
| ◄    | `\u{25C4}` | BLACK LEFT-POINTING POINTER                   |
| ◅    | `\u{25C5}` | WHITE LEFT-POINTING POINTER                   |
| ◆    | `\u{25C6}` | BLACK DIAMOND                                 |
| ◇    | `\u{25C7}` | WHITE DIAMOND                                 |
| ◈    | `\u{25C8}` | WHITE DIAMOND CONTAINING BLACK SMALL DIAMOND  |
| ◉    | `\u{25C9}` | FISHEYE                                       |
| ◊    | `\u{25CA}` | LOZENGE                                       |
| ○    | `\u{25CB}` | WHITE CIRCLE                                  |
| ◌    | `\u{25CC}` | DOTTED CIRCLE                                 |
| ◍    | `\u{25CD}` | CIRCLE WITH VERTICAL FILL                     |
| ◎    | `\u{25CE}` | BULLSEYE                                      |
| ●    | `\u{25CF}` | BLACK CIRCLE                                  |
| ◐    | `\u{25D0}` | CIRCLE WITH LEFT HALF BLACK                   |
| ◑    | `\u{25D1}` | CIRCLE WITH RIGHT HALF BLACK                  |
| ◒    | `\u{25D2}` | CIRCLE WITH LOWER HALF BLACK                  |
| ◓    | `\u{25D3}` | CIRCLE WITH UPPER HALF BLACK                  |
| ◔    | `\u{25D4}` | CIRCLE WITH UPPER RIGHT QUADRANT BLACK        |
| ◕    | `\u{25D5}` | CIRCLE WITH ALL BUT UPPER LEFT QUADRANT BLACK |
| ◖    | `\u{25D6}` | LEFT HALF BLACK CIRCLE                        |
| ◗    | `\u{25D7}` | RIGHT HALF BLACK CIRCLE                       |
| ◘    | `\u{25D8}` | INVERSE BULLET                                |
| ◙    | `\u{25D9}` | INVERSE WHITE CIRCLE                          |
| ◚    | `\u{25DA}` | UPPER HALF INVERSE WHITE CIRCLE               |
| ◛    | `\u{25DB}` | LOWER HALF INVERSE WHITE CIRCLE               |
| ◜    | `\u{25DC}` | UPPER LEFT QUADRANT CIRCULAR ARC              |
| ◝    | `\u{25DD}` | UPPER RIGHT QUADRANT CIRCULAR ARC             |
| ◞    | `\u{25DE}` | LOWER RIGHT QUADRANT CIRCULAR ARC             |
| ◟    | `\u{25DF}` | LOWER LEFT QUADRANT CIRCULAR ARC              |
| ◠    | `\u{25E0}` | UPPER HALF CIRCLE                             |
| ◡    | `\u{25E1}` | LOWER HALF CIRCLE                             |
| ◢    | `\u{25E2}` | BLACK LOWER RIGHT TRIANGLE                    |
| ◣    | `\u{25E3}` | BLACK LOWER LEFT TRIANGLE                     |
| ◤    | `\u{25E4}` | BLACK UPPER LEFT TRIANGLE                     |
| ◥    | `\u{25E5}` | BLACK UPPER RIGHT TRIANGLE                    |
| ◦    | `\u{25E6}` | WHITE BULLET                                  |
| ◧    | `\u{25E7}` | SQUARE WITH LEFT HALF BLACK                   |
| ◨    | `\u{25E8}` | SQUARE WITH RIGHT HALF BLACK                  |
| ◩    | `\u{25E9}` | SQUARE WITH UPPER LEFT DIAGONAL HALF BLACK    |
| ◪    | `\u{25EA}` | SQUARE WITH LOWER RIGHT DIAGONAL HALF BLACK   |
| ◫    | `\u{25EB}` | WHITE SQUARE WITH VERTICAL BISECTING LINE     |
| ◬    | `\u{25EC}` | WHITE UP-POINTING TRIANGLE WITH DOT           |
| ◭    | `\u{25ED}` | UP-POINTING TRIANGLE WITH LEFT HALF BLACK     |
| ◮    | `\u{25EE}` | UP-POINTING TRIANGLE WITH RIGHT HALF BLACK    |
| ◯    | `\u{25EF}` | LARGE CIRCLE                                  |
| ◰    | `\u{25F0}` | WHITE SQUARE WITH UPPER LEFT QUADRANT         |
| ◱    | `\u{25F1}` | WHITE SQUARE WITH LOWER LEFT QUADRANT         |
| ◲    | `\u{25F2}` | WHITE SQUARE WITH LOWER RIGHT QUADRANT        |
| ◳    | `\u{25F3}` | WHITE SQUARE WITH UPPER RIGHT QUADRANT        |
| ◴    | `\u{25F4}` | WHITE CIRCLE WITH UPPER LEFT QUADRANT         |
| ◵    | `\u{25F5}` | WHITE CIRCLE WITH LOWER LEFT QUADRANT         |
| ◶    | `\u{25F6}` | WHITE CIRCLE WITH LOWER RIGHT QUADRANT        |
| ◷    | `\u{25F7}` | WHITE CIRCLE WITH UPPER RIGHT QUADRANT        |
| ◸    | `\u{25F8}` | UPPER LEFT TRIANGLE                           |
| ◹    | `\u{25F9}` | UPPER RIGHT TRIANGLE                          |
| ◺    | `\u{25FA}` | LOWER LEFT TRIANGLE                           |
| ◻    | `\u{25FB}` | WHITE MEDIUM SQUARE                           |
| ◼    | `\u{25FC}` | BLACK MEDIUM SQUARE                           |
| ◽   | `\u{25FD}` | WHITE MEDIUM SMALL SQUARE                     |
| ◾   | `\u{25FE}` | BLACK MEDIUM SMALL SQUARE                     |
| ◿    | `\u{25FF}` | LOWER RIGHT TRIANGLE                          |

## Mathematical Operators (256)

| Char | Escape     | Name                                                           |
|------|------------|----------------------------------------------------------------|
| ∀    | `\u{2200}` | FOR ALL                                                        |
| ∁    | `\u{2201}` | COMPLEMENT                                                     |
| ∂    | `\u{2202}` | PARTIAL DIFFERENTIAL                                           |
| ∃    | `\u{2203}` | THERE EXISTS                                                   |
| ∄    | `\u{2204}` | THERE DOES NOT EXIST                                           |
| ∅    | `\u{2205}` | EMPTY SET                                                      |
| ∆    | `\u{2206}` | INCREMENT                                                      |
| ∇    | `\u{2207}` | NABLA                                                          |
| ∈    | `\u{2208}` | ELEMENT OF                                                     |
| ∉    | `\u{2209}` | NOT AN ELEMENT OF                                              |
| ∊    | `\u{220A}` | SMALL ELEMENT OF                                               |
| ∋    | `\u{220B}` | CONTAINS AS MEMBER                                             |
| ∌    | `\u{220C}` | DOES NOT CONTAIN AS MEMBER                                     |
| ∍    | `\u{220D}` | SMALL CONTAINS AS MEMBER                                       |
| ∎    | `\u{220E}` | END OF PROOF                                                   |
| ∏    | `\u{220F}` | N-ARY PRODUCT                                                  |
| ∐    | `\u{2210}` | N-ARY COPRODUCT                                                |
| ∑    | `\u{2211}` | N-ARY SUMMATION                                                |
| −    | `\u{2212}` | MINUS SIGN                                                     |
| ∓    | `\u{2213}` | MINUS-OR-PLUS SIGN                                             |
| ∔    | `\u{2214}` | DOT PLUS                                                       |
| ∕    | `\u{2215}` | DIVISION SLASH                                                 |
| ∖    | `\u{2216}` | SET MINUS                                                      |
| ∗    | `\u{2217}` | ASTERISK OPERATOR                                              |
| ∘    | `\u{2218}` | RING OPERATOR                                                  |
| ∙    | `\u{2219}` | BULLET OPERATOR                                                |
| √    | `\u{221A}` | SQUARE ROOT                                                    |
| ∛    | `\u{221B}` | CUBE ROOT                                                      |
| ∜    | `\u{221C}` | FOURTH ROOT                                                    |
| ∝    | `\u{221D}` | PROPORTIONAL TO                                                |
| ∞    | `\u{221E}` | INFINITY                                                       |
| ∟    | `\u{221F}` | RIGHT ANGLE                                                    |
| ∠    | `\u{2220}` | ANGLE                                                          |
| ∡    | `\u{2221}` | MEASURED ANGLE                                                 |
| ∢    | `\u{2222}` | SPHERICAL ANGLE                                                |
| ∣    | `\u{2223}` | DIVIDES                                                        |
| ∤    | `\u{2224}` | DOES NOT DIVIDE                                                |
| ∥    | `\u{2225}` | PARALLEL TO                                                    |
| ∦    | `\u{2226}` | NOT PARALLEL TO                                                |
| ∧    | `\u{2227}` | LOGICAL AND                                                    |
| ∨    | `\u{2228}` | LOGICAL OR                                                     |
| ∩    | `\u{2229}` | INTERSECTION                                                   |
| ∪    | `\u{222A}` | UNION                                                          |
| ∫    | `\u{222B}` | INTEGRAL                                                       |
| ∬    | `\u{222C}` | DOUBLE INTEGRAL                                                |
| ∭    | `\u{222D}` | TRIPLE INTEGRAL                                                |
| ∮    | `\u{222E}` | CONTOUR INTEGRAL                                               |
| ∯    | `\u{222F}` | SURFACE INTEGRAL                                               |
| ∰    | `\u{2230}` | VOLUME INTEGRAL                                                |
| ∱    | `\u{2231}` | CLOCKWISE INTEGRAL                                             |
| ∲    | `\u{2232}` | CLOCKWISE CONTOUR INTEGRAL                                     |
| ∳    | `\u{2233}` | ANTICLOCKWISE CONTOUR INTEGRAL                                 |
| ∴    | `\u{2234}` | THEREFORE                                                      |
| ∵    | `\u{2235}` | BECAUSE                                                        |
| ∶    | `\u{2236}` | RATIO                                                          |
| ∷    | `\u{2237}` | PROPORTION                                                     |
| ∸    | `\u{2238}` | DOT MINUS                                                      |
| ∹    | `\u{2239}` | EXCESS                                                         |
| ∺    | `\u{223A}` | GEOMETRIC PROPORTION                                           |
| ∻    | `\u{223B}` | HOMOTHETIC                                                     |
| ∼    | `\u{223C}` | TILDE OPERATOR                                                 |
| ∽    | `\u{223D}` | REVERSED TILDE                                                 |
| ∾    | `\u{223E}` | INVERTED LAZY S                                                |
| ∿    | `\u{223F}` | SINE WAVE                                                      |
| ≀    | `\u{2240}` | WREATH PRODUCT                                                 |
| ≁    | `\u{2241}` | NOT TILDE                                                      |
| ≂    | `\u{2242}` | MINUS TILDE                                                    |
| ≃    | `\u{2243}` | ASYMPTOTICALLY EQUAL TO                                        |
| ≄    | `\u{2244}` | NOT ASYMPTOTICALLY EQUAL TO                                    |
| ≅    | `\u{2245}` | APPROXIMATELY EQUAL TO                                         |
| ≆    | `\u{2246}` | APPROXIMATELY BUT NOT ACTUALLY EQUAL TO                        |
| ≇    | `\u{2247}` | NEITHER APPROXIMATELY NOR ACTUALLY EQUAL TO                    |
| ≈    | `\u{2248}` | ALMOST EQUAL TO                                                |
| ≉    | `\u{2249}` | NOT ALMOST EQUAL TO                                            |
| ≊    | `\u{224A}` | ALMOST EQUAL OR EQUAL TO                                       |
| ≋    | `\u{224B}` | TRIPLE TILDE                                                   |
| ≌    | `\u{224C}` | ALL EQUAL TO                                                   |
| ≍    | `\u{224D}` | EQUIVALENT TO                                                  |
| ≎    | `\u{224E}` | GEOMETRICALLY EQUIVALENT TO                                    |
| ≏    | `\u{224F}` | DIFFERENCE BETWEEN                                             |
| ≐    | `\u{2250}` | APPROACHES THE LIMIT                                           |
| ≑    | `\u{2251}` | GEOMETRICALLY EQUAL TO                                         |
| ≒    | `\u{2252}` | APPROXIMATELY EQUAL TO OR THE IMAGE OF                         |
| ≓    | `\u{2253}` | IMAGE OF OR APPROXIMATELY EQUAL TO                             |
| ≔    | `\u{2254}` | COLON EQUALS                                                   |
| ≕    | `\u{2255}` | EQUALS COLON                                                   |
| ≖    | `\u{2256}` | RING IN EQUAL TO                                               |
| ≗    | `\u{2257}` | RING EQUAL TO                                                  |
| ≘    | `\u{2258}` | CORRESPONDS TO                                                 |
| ≙    | `\u{2259}` | ESTIMATES                                                      |
| ≚    | `\u{225A}` | EQUIANGULAR TO                                                 |
| ≛    | `\u{225B}` | STAR EQUALS                                                    |
| ≜    | `\u{225C}` | DELTA EQUAL TO                                                 |
| ≝    | `\u{225D}` | EQUAL TO BY DEFINITION                                         |
| ≞    | `\u{225E}` | MEASURED BY                                                    |
| ≟    | `\u{225F}` | QUESTIONED EQUAL TO                                            |
| ≠    | `\u{2260}` | NOT EQUAL TO                                                   |
| ≡    | `\u{2261}` | IDENTICAL TO                                                   |
| ≢    | `\u{2262}` | NOT IDENTICAL TO                                               |
| ≣    | `\u{2263}` | STRICTLY EQUIVALENT TO                                         |
| ≤    | `\u{2264}` | LESS-THAN OR EQUAL TO                                          |
| ≥    | `\u{2265}` | GREATER-THAN OR EQUAL TO                                       |
| ≦    | `\u{2266}` | LESS-THAN OVER EQUAL TO                                        |
| ≧    | `\u{2267}` | GREATER-THAN OVER EQUAL TO                                     |
| ≨    | `\u{2268}` | LESS-THAN BUT NOT EQUAL TO                                     |
| ≩    | `\u{2269}` | GREATER-THAN BUT NOT EQUAL TO                                  |
| ≪    | `\u{226A}` | MUCH LESS-THAN                                                 |
| ≫    | `\u{226B}` | MUCH GREATER-THAN                                              |
| ≬    | `\u{226C}` | BETWEEN                                                        |
| ≭    | `\u{226D}` | NOT EQUIVALENT TO                                              |
| ≮    | `\u{226E}` | NOT LESS-THAN                                                  |
| ≯    | `\u{226F}` | NOT GREATER-THAN                                               |
| ≰    | `\u{2270}` | NEITHER LESS-THAN NOR EQUAL TO                                 |
| ≱    | `\u{2271}` | NEITHER GREATER-THAN NOR EQUAL TO                              |
| ≲    | `\u{2272}` | LESS-THAN OR EQUIVALENT TO                                     |
| ≳    | `\u{2273}` | GREATER-THAN OR EQUIVALENT TO                                  |
| ≴    | `\u{2274}` | NEITHER LESS-THAN NOR EQUIVALENT TO                            |
| ≵    | `\u{2275}` | NEITHER GREATER-THAN NOR EQUIVALENT TO                         |
| ≶    | `\u{2276}` | LESS-THAN OR GREATER-THAN                                      |
| ≷    | `\u{2277}` | GREATER-THAN OR LESS-THAN                                      |
| ≸    | `\u{2278}` | NEITHER LESS-THAN NOR GREATER-THAN                             |
| ≹    | `\u{2279}` | NEITHER GREATER-THAN NOR LESS-THAN                             |
| ≺    | `\u{227A}` | PRECEDES                                                       |
| ≻    | `\u{227B}` | SUCCEEDS                                                       |
| ≼    | `\u{227C}` | PRECEDES OR EQUAL TO                                           |
| ≽    | `\u{227D}` | SUCCEEDS OR EQUAL TO                                           |
| ≾    | `\u{227E}` | PRECEDES OR EQUIVALENT TO                                      |
| ≿    | `\u{227F}` | SUCCEEDS OR EQUIVALENT TO                                      |
| ⊀    | `\u{2280}` | DOES NOT PRECEDE                                               |
| ⊁    | `\u{2281}` | DOES NOT SUCCEED                                               |
| ⊂    | `\u{2282}` | SUBSET OF                                                      |
| ⊃    | `\u{2283}` | SUPERSET OF                                                    |
| ⊄    | `\u{2284}` | NOT A SUBSET OF                                                |
| ⊅    | `\u{2285}` | NOT A SUPERSET OF                                              |
| ⊆    | `\u{2286}` | SUBSET OF OR EQUAL TO                                          |
| ⊇    | `\u{2287}` | SUPERSET OF OR EQUAL TO                                        |
| ⊈    | `\u{2288}` | NEITHER A SUBSET OF NOR EQUAL TO                               |
| ⊉    | `\u{2289}` | NEITHER A SUPERSET OF NOR EQUAL TO                             |
| ⊊    | `\u{228A}` | SUBSET OF WITH NOT EQUAL TO                                    |
| ⊋    | `\u{228B}` | SUPERSET OF WITH NOT EQUAL TO                                  |
| ⊌    | `\u{228C}` | MULTISET                                                       |
| ⊍    | `\u{228D}` | MULTISET MULTIPLICATION                                        |
| ⊎    | `\u{228E}` | MULTISET UNION                                                 |
| ⊏    | `\u{228F}` | SQUARE IMAGE OF                                                |
| ⊐    | `\u{2290}` | SQUARE ORIGINAL OF                                             |
| ⊑    | `\u{2291}` | SQUARE IMAGE OF OR EQUAL TO                                    |
| ⊒    | `\u{2292}` | SQUARE ORIGINAL OF OR EQUAL TO                                 |
| ⊓    | `\u{2293}` | SQUARE CAP                                                     |
| ⊔    | `\u{2294}` | SQUARE CUP                                                     |
| ⊕    | `\u{2295}` | CIRCLED PLUS                                                   |
| ⊖    | `\u{2296}` | CIRCLED MINUS                                                  |
| ⊗    | `\u{2297}` | CIRCLED TIMES                                                  |
| ⊘    | `\u{2298}` | CIRCLED DIVISION SLASH                                         |
| ⊙    | `\u{2299}` | CIRCLED DOT OPERATOR                                           |
| ⊚    | `\u{229A}` | CIRCLED RING OPERATOR                                          |
| ⊛    | `\u{229B}` | CIRCLED ASTERISK OPERATOR                                      |
| ⊜    | `\u{229C}` | CIRCLED EQUALS                                                 |
| ⊝    | `\u{229D}` | CIRCLED DASH                                                   |
| ⊞    | `\u{229E}` | SQUARED PLUS                                                   |
| ⊟    | `\u{229F}` | SQUARED MINUS                                                  |
| ⊠    | `\u{22A0}` | SQUARED TIMES                                                  |
| ⊡    | `\u{22A1}` | SQUARED DOT OPERATOR                                           |
| ⊢    | `\u{22A2}` | RIGHT TACK                                                     |
| ⊣    | `\u{22A3}` | LEFT TACK                                                      |
| ⊤    | `\u{22A4}` | DOWN TACK                                                      |
| ⊥    | `\u{22A5}` | UP TACK                                                        |
| ⊦    | `\u{22A6}` | ASSERTION                                                      |
| ⊧    | `\u{22A7}` | MODELS                                                         |
| ⊨    | `\u{22A8}` | TRUE                                                           |
| ⊩    | `\u{22A9}` | FORCES                                                         |
| ⊪    | `\u{22AA}` | TRIPLE VERTICAL BAR RIGHT TURNSTILE                            |
| ⊫    | `\u{22AB}` | DOUBLE VERTICAL BAR DOUBLE RIGHT TURNSTILE                     |
| ⊬    | `\u{22AC}` | DOES NOT PROVE                                                 |
| ⊭    | `\u{22AD}` | NOT TRUE                                                       |
| ⊮    | `\u{22AE}` | DOES NOT FORCE                                                 |
| ⊯    | `\u{22AF}` | NEGATED DOUBLE VERTICAL BAR DOUBLE RIGHT TURNSTILE             |
| ⊰    | `\u{22B0}` | PRECEDES UNDER RELATION                                        |
| ⊱    | `\u{22B1}` | SUCCEEDS UNDER RELATION                                        |
| ⊲    | `\u{22B2}` | NORMAL SUBGROUP OF                                             |
| ⊳    | `\u{22B3}` | CONTAINS AS NORMAL SUBGROUP                                    |
| ⊴    | `\u{22B4}` | NORMAL SUBGROUP OF OR EQUAL TO                                 |
| ⊵    | `\u{22B5}` | CONTAINS AS NORMAL SUBGROUP OR EQUAL TO                        |
| ⊶    | `\u{22B6}` | ORIGINAL OF                                                    |
| ⊷    | `\u{22B7}` | IMAGE OF                                                       |
| ⊸    | `\u{22B8}` | MULTIMAP                                                       |
| ⊹    | `\u{22B9}` | HERMITIAN CONJUGATE MATRIX                                     |
| ⊺    | `\u{22BA}` | INTERCALATE                                                    |
| ⊻    | `\u{22BB}` | XOR                                                            |
| ⊼    | `\u{22BC}` | NAND                                                           |
| ⊽    | `\u{22BD}` | NOR                                                            |
| ⊾    | `\u{22BE}` | RIGHT ANGLE WITH ARC                                           |
| ⊿    | `\u{22BF}` | RIGHT TRIANGLE                                                 |
| ⋀    | `\u{22C0}` | N-ARY LOGICAL AND                                              |
| ⋁    | `\u{22C1}` | N-ARY LOGICAL OR                                               |
| ⋂    | `\u{22C2}` | N-ARY INTERSECTION                                             |
| ⋃    | `\u{22C3}` | N-ARY UNION                                                    |
| ⋄    | `\u{22C4}` | DIAMOND OPERATOR                                               |
| ⋅    | `\u{22C5}` | DOT OPERATOR                                                   |
| ⋆    | `\u{22C6}` | STAR OPERATOR                                                  |
| ⋇    | `\u{22C7}` | DIVISION TIMES                                                 |
| ⋈    | `\u{22C8}` | BOWTIE                                                         |
| ⋉    | `\u{22C9}` | LEFT NORMAL FACTOR SEMIDIRECT PRODUCT                          |
| ⋊    | `\u{22CA}` | RIGHT NORMAL FACTOR SEMIDIRECT PRODUCT                         |
| ⋋    | `\u{22CB}` | LEFT SEMIDIRECT PRODUCT                                        |
| ⋌    | `\u{22CC}` | RIGHT SEMIDIRECT PRODUCT                                       |
| ⋍    | `\u{22CD}` | REVERSED TILDE EQUALS                                          |
| ⋎    | `\u{22CE}` | CURLY LOGICAL OR                                               |
| ⋏    | `\u{22CF}` | CURLY LOGICAL AND                                              |
| ⋐    | `\u{22D0}` | DOUBLE SUBSET                                                  |
| ⋑    | `\u{22D1}` | DOUBLE SUPERSET                                                |
| ⋒    | `\u{22D2}` | DOUBLE INTERSECTION                                            |
| ⋓    | `\u{22D3}` | DOUBLE UNION                                                   |
| ⋔    | `\u{22D4}` | PITCHFORK                                                      |
| ⋕    | `\u{22D5}` | EQUAL AND PARALLEL TO                                          |
| ⋖    | `\u{22D6}` | LESS-THAN WITH DOT                                             |
| ⋗    | `\u{22D7}` | GREATER-THAN WITH DOT                                          |
| ⋘    | `\u{22D8}` | VERY MUCH LESS-THAN                                            |
| ⋙    | `\u{22D9}` | VERY MUCH GREATER-THAN                                         |
| ⋚    | `\u{22DA}` | LESS-THAN EQUAL TO OR GREATER-THAN                             |
| ⋛    | `\u{22DB}` | GREATER-THAN EQUAL TO OR LESS-THAN                             |
| ⋜    | `\u{22DC}` | EQUAL TO OR LESS-THAN                                          |
| ⋝    | `\u{22DD}` | EQUAL TO OR GREATER-THAN                                       |
| ⋞    | `\u{22DE}` | EQUAL TO OR PRECEDES                                           |
| ⋟    | `\u{22DF}` | EQUAL TO OR SUCCEEDS                                           |
| ⋠    | `\u{22E0}` | DOES NOT PRECEDE OR EQUAL                                      |
| ⋡    | `\u{22E1}` | DOES NOT SUCCEED OR EQUAL                                      |
| ⋢    | `\u{22E2}` | NOT SQUARE IMAGE OF OR EQUAL TO                                |
| ⋣    | `\u{22E3}` | NOT SQUARE ORIGINAL OF OR EQUAL TO                             |
| ⋤    | `\u{22E4}` | SQUARE IMAGE OF OR NOT EQUAL TO                                |
| ⋥    | `\u{22E5}` | SQUARE ORIGINAL OF OR NOT EQUAL TO                             |
| ⋦    | `\u{22E6}` | LESS-THAN BUT NOT EQUIVALENT TO                                |
| ⋧    | `\u{22E7}` | GREATER-THAN BUT NOT EQUIVALENT TO                             |
| ⋨    | `\u{22E8}` | PRECEDES BUT NOT EQUIVALENT TO                                 |
| ⋩    | `\u{22E9}` | SUCCEEDS BUT NOT EQUIVALENT TO                                 |
| ⋪    | `\u{22EA}` | NOT NORMAL SUBGROUP OF                                         |
| ⋫    | `\u{22EB}` | DOES NOT CONTAIN AS NORMAL SUBGROUP                            |
| ⋬    | `\u{22EC}` | NOT NORMAL SUBGROUP OF OR EQUAL TO                             |
| ⋭    | `\u{22ED}` | DOES NOT CONTAIN AS NORMAL SUBGROUP OR EQUAL                   |
| ⋮    | `\u{22EE}` | VERTICAL ELLIPSIS                                              |
| ⋯    | `\u{22EF}` | MIDLINE HORIZONTAL ELLIPSIS                                    |
| ⋰    | `\u{22F0}` | UP RIGHT DIAGONAL ELLIPSIS                                     |
| ⋱    | `\u{22F1}` | DOWN RIGHT DIAGONAL ELLIPSIS                                   |
| ⋲    | `\u{22F2}` | ELEMENT OF WITH LONG HORIZONTAL STROKE                         |
| ⋳    | `\u{22F3}` | ELEMENT OF WITH VERTICAL BAR AT END OF HORIZONTAL STROKE       |
| ⋴    | `\u{22F4}` | SMALL ELEMENT OF WITH VERTICAL BAR AT END OF HORIZONTAL STROKE |
| ⋵    | `\u{22F5}` | ELEMENT OF WITH DOT ABOVE                                      |
| ⋶    | `\u{22F6}` | ELEMENT OF WITH OVERBAR                                        |
| ⋷    | `\u{22F7}` | SMALL ELEMENT OF WITH OVERBAR                                  |
| ⋸    | `\u{22F8}` | ELEMENT OF WITH UNDERBAR                                       |
| ⋹    | `\u{22F9}` | ELEMENT OF WITH TWO HORIZONTAL STROKES                         |
| ⋺    | `\u{22FA}` | CONTAINS WITH LONG HORIZONTAL STROKE                           |
| ⋻    | `\u{22FB}` | CONTAINS WITH VERTICAL BAR AT END OF HORIZONTAL STROKE         |
| ⋼    | `\u{22FC}` | SMALL CONTAINS WITH VERTICAL BAR AT END OF HORIZONTAL STROKE   |
| ⋽    | `\u{22FD}` | CONTAINS WITH OVERBAR                                          |
| ⋾    | `\u{22FE}` | SMALL CONTAINS WITH OVERBAR                                    |
| ⋿    | `\u{22FF}` | Z NOTATION BAG MEMBERSHIP                                      |

## Greek and Coptic (144)

| Char | Escape     | Name                                                |
|------|------------|-----------------------------------------------------|
| Ͱ    | `\u{0370}` | GREEK CAPITAL LETTER HETA                           |
| ͱ    | `\u{0371}` | GREEK SMALL LETTER HETA                             |
| Ͳ    | `\u{0372}` | GREEK CAPITAL LETTER ARCHAIC SAMPI                  |
| ͳ    | `\u{0373}` | GREEK SMALL LETTER ARCHAIC SAMPI                    |
| ʹ    | `\u{0374}` | GREEK NUMERAL SIGN                                  |
| ͵    | `\u{0375}` | GREEK LOWER NUMERAL SIGN                            |
| Ͷ    | `\u{0376}` | GREEK CAPITAL LETTER PAMPHYLIAN DIGAMMA             |
| ͷ    | `\u{0377}` | GREEK SMALL LETTER PAMPHYLIAN DIGAMMA               |
| ͸    | `\u{0378}` |                                                     |
| ͹    | `\u{0379}` |                                                     |
| ͺ    | `\u{037A}` | GREEK YPOGEGRAMMENI                                 |
| ͻ    | `\u{037B}` | GREEK SMALL REVERSED LUNATE SIGMA SYMBOL            |
| ͼ    | `\u{037C}` | GREEK SMALL DOTTED LUNATE SIGMA SYMBOL              |
| ͽ    | `\u{037D}` | GREEK SMALL REVERSED DOTTED LUNATE SIGMA SYMBOL     |
| ;    | `\u{037E}` | GREEK QUESTION MARK                                 |
| Ϳ    | `\u{037F}` | GREEK CAPITAL LETTER YOT                            |
| ΀    | `\u{0380}` |                                                     |
| ΁    | `\u{0381}` |                                                     |
| ΂    | `\u{0382}` |                                                     |
| ΃    | `\u{0383}` |                                                     |
| ΄    | `\u{0384}` | GREEK TONOS                                         |
| ΅    | `\u{0385}` | GREEK DIALYTIKA TONOS                               |
| Ά    | `\u{0386}` | GREEK CAPITAL LETTER ALPHA WITH TONOS               |
| ·    | `\u{0387}` | GREEK ANO TELEIA                                    |
| Έ    | `\u{0388}` | GREEK CAPITAL LETTER EPSILON WITH TONOS             |
| Ή    | `\u{0389}` | GREEK CAPITAL LETTER ETA WITH TONOS                 |
| Ί    | `\u{038A}` | GREEK CAPITAL LETTER IOTA WITH TONOS                |
| ΋    | `\u{038B}` |                                                     |
| Ό    | `\u{038C}` | GREEK CAPITAL LETTER OMICRON WITH TONOS             |
| ΍    | `\u{038D}` |                                                     |
| Ύ    | `\u{038E}` | GREEK CAPITAL LETTER UPSILON WITH TONOS             |
| Ώ    | `\u{038F}` | GREEK CAPITAL LETTER OMEGA WITH TONOS               |
| ΐ    | `\u{0390}` | GREEK SMALL LETTER IOTA WITH DIALYTIKA AND TONOS    |
| Α    | `\u{0391}` | GREEK CAPITAL LETTER ALPHA                          |
| Β    | `\u{0392}` | GREEK CAPITAL LETTER BETA                           |
| Γ    | `\u{0393}` | GREEK CAPITAL LETTER GAMMA                          |
| Δ    | `\u{0394}` | GREEK CAPITAL LETTER DELTA                          |
| Ε    | `\u{0395}` | GREEK CAPITAL LETTER EPSILON                        |
| Ζ    | `\u{0396}` | GREEK CAPITAL LETTER ZETA                           |
| Η    | `\u{0397}` | GREEK CAPITAL LETTER ETA                            |
| Θ    | `\u{0398}` | GREEK CAPITAL LETTER THETA                          |
| Ι    | `\u{0399}` | GREEK CAPITAL LETTER IOTA                           |
| Κ    | `\u{039A}` | GREEK CAPITAL LETTER KAPPA                          |
| Λ    | `\u{039B}` | GREEK CAPITAL LETTER LAMDA                          |
| Μ    | `\u{039C}` | GREEK CAPITAL LETTER MU                             |
| Ν    | `\u{039D}` | GREEK CAPITAL LETTER NU                             |
| Ξ    | `\u{039E}` | GREEK CAPITAL LETTER XI                             |
| Ο    | `\u{039F}` | GREEK CAPITAL LETTER OMICRON                        |
| Π    | `\u{03A0}` | GREEK CAPITAL LETTER PI                             |
| Ρ    | `\u{03A1}` | GREEK CAPITAL LETTER RHO                            |
| ΢    | `\u{03A2}` |                                                     |
| Σ    | `\u{03A3}` | GREEK CAPITAL LETTER SIGMA                          |
| Τ    | `\u{03A4}` | GREEK CAPITAL LETTER TAU                            |
| Υ    | `\u{03A5}` | GREEK CAPITAL LETTER UPSILON                        |
| Φ    | `\u{03A6}` | GREEK CAPITAL LETTER PHI                            |
| Χ    | `\u{03A7}` | GREEK CAPITAL LETTER CHI                            |
| Ψ    | `\u{03A8}` | GREEK CAPITAL LETTER PSI                            |
| Ω    | `\u{03A9}` | GREEK CAPITAL LETTER OMEGA                          |
| Ϊ    | `\u{03AA}` | GREEK CAPITAL LETTER IOTA WITH DIALYTIKA            |
| Ϋ    | `\u{03AB}` | GREEK CAPITAL LETTER UPSILON WITH DIALYTIKA         |
| ά    | `\u{03AC}` | GREEK SMALL LETTER ALPHA WITH TONOS                 |
| έ    | `\u{03AD}` | GREEK SMALL LETTER EPSILON WITH TONOS               |
| ή    | `\u{03AE}` | GREEK SMALL LETTER ETA WITH TONOS                   |
| ί    | `\u{03AF}` | GREEK SMALL LETTER IOTA WITH TONOS                  |
| ΰ    | `\u{03B0}` | GREEK SMALL LETTER UPSILON WITH DIALYTIKA AND TONOS |
| α    | `\u{03B1}` | GREEK SMALL LETTER ALPHA                            |
| β    | `\u{03B2}` | GREEK SMALL LETTER BETA                             |
| γ    | `\u{03B3}` | GREEK SMALL LETTER GAMMA                            |
| δ    | `\u{03B4}` | GREEK SMALL LETTER DELTA                            |
| ε    | `\u{03B5}` | GREEK SMALL LETTER EPSILON                          |
| ζ    | `\u{03B6}` | GREEK SMALL LETTER ZETA                             |
| η    | `\u{03B7}` | GREEK SMALL LETTER ETA                              |
| θ    | `\u{03B8}` | GREEK SMALL LETTER THETA                            |
| ι    | `\u{03B9}` | GREEK SMALL LETTER IOTA                             |
| κ    | `\u{03BA}` | GREEK SMALL LETTER KAPPA                            |
| λ    | `\u{03BB}` | GREEK SMALL LETTER LAMDA                            |
| μ    | `\u{03BC}` | GREEK SMALL LETTER MU                               |
| ν    | `\u{03BD}` | GREEK SMALL LETTER NU                               |
| ξ    | `\u{03BE}` | GREEK SMALL LETTER XI                               |
| ο    | `\u{03BF}` | GREEK SMALL LETTER OMICRON                          |
| π    | `\u{03C0}` | GREEK SMALL LETTER PI                               |
| ρ    | `\u{03C1}` | GREEK SMALL LETTER RHO                              |
| ς    | `\u{03C2}` | GREEK SMALL LETTER FINAL SIGMA                      |
| σ    | `\u{03C3}` | GREEK SMALL LETTER SIGMA                            |
| τ    | `\u{03C4}` | GREEK SMALL LETTER TAU                              |
| υ    | `\u{03C5}` | GREEK SMALL LETTER UPSILON                          |
| φ    | `\u{03C6}` | GREEK SMALL LETTER PHI                              |
| χ    | `\u{03C7}` | GREEK SMALL LETTER CHI                              |
| ψ    | `\u{03C8}` | GREEK SMALL LETTER PSI                              |
| ω    | `\u{03C9}` | GREEK SMALL LETTER OMEGA                            |
| ϊ    | `\u{03CA}` | GREEK SMALL LETTER IOTA WITH DIALYTIKA              |
| ϋ    | `\u{03CB}` | GREEK SMALL LETTER UPSILON WITH DIALYTIKA           |
| ό    | `\u{03CC}` | GREEK SMALL LETTER OMICRON WITH TONOS               |
| ύ    | `\u{03CD}` | GREEK SMALL LETTER UPSILON WITH TONOS               |
| ώ    | `\u{03CE}` | GREEK SMALL LETTER OMEGA WITH TONOS                 |
| Ϗ    | `\u{03CF}` | GREEK CAPITAL KAI SYMBOL                            |
| ϐ    | `\u{03D0}` | GREEK BETA SYMBOL                                   |
| ϑ    | `\u{03D1}` | GREEK THETA SYMBOL                                  |
| ϒ    | `\u{03D2}` | GREEK UPSILON WITH HOOK SYMBOL                      |
| ϓ    | `\u{03D3}` | GREEK UPSILON WITH ACUTE AND HOOK SYMBOL            |
| ϔ    | `\u{03D4}` | GREEK UPSILON WITH DIAERESIS AND HOOK SYMBOL        |
| ϕ    | `\u{03D5}` | GREEK PHI SYMBOL                                    |
| ϖ    | `\u{03D6}` | GREEK PI SYMBOL                                     |
| ϗ    | `\u{03D7}` | GREEK KAI SYMBOL                                    |
| Ϙ    | `\u{03D8}` | GREEK LETTER ARCHAIC KOPPA                          |
| ϙ    | `\u{03D9}` | GREEK SMALL LETTER ARCHAIC KOPPA                    |
| Ϛ    | `\u{03DA}` | GREEK LETTER STIGMA                                 |
| ϛ    | `\u{03DB}` | GREEK SMALL LETTER STIGMA                           |
| Ϝ    | `\u{03DC}` | GREEK LETTER DIGAMMA                                |
| ϝ    | `\u{03DD}` | GREEK SMALL LETTER DIGAMMA                          |
| Ϟ    | `\u{03DE}` | GREEK LETTER KOPPA                                  |
| ϟ    | `\u{03DF}` | GREEK SMALL LETTER KOPPA                            |
| Ϡ    | `\u{03E0}` | GREEK LETTER SAMPI                                  |
| ϡ    | `\u{03E1}` | GREEK SMALL LETTER SAMPI                            |
| Ϣ    | `\u{03E2}` | COPTIC CAPITAL LETTER SHEI                          |
| ϣ    | `\u{03E3}` | COPTIC SMALL LETTER SHEI                            |
| Ϥ    | `\u{03E4}` | COPTIC CAPITAL LETTER FEI                           |
| ϥ    | `\u{03E5}` | COPTIC SMALL LETTER FEI                             |
| Ϧ    | `\u{03E6}` | COPTIC CAPITAL LETTER KHEI                          |
| ϧ    | `\u{03E7}` | COPTIC SMALL LETTER KHEI                            |
| Ϩ    | `\u{03E8}` | COPTIC CAPITAL LETTER HORI                          |
| ϩ    | `\u{03E9}` | COPTIC SMALL LETTER HORI                            |
| Ϫ    | `\u{03EA}` | COPTIC CAPITAL LETTER GANGIA                        |
| ϫ    | `\u{03EB}` | COPTIC SMALL LETTER GANGIA                          |
| Ϭ    | `\u{03EC}` | COPTIC CAPITAL LETTER SHIMA                         |
| ϭ    | `\u{03ED}` | COPTIC SMALL LETTER SHIMA                           |
| Ϯ    | `\u{03EE}` | COPTIC CAPITAL LETTER DEI                           |
| ϯ    | `\u{03EF}` | COPTIC SMALL LETTER DEI                             |
| ϰ    | `\u{03F0}` | GREEK KAPPA SYMBOL                                  |
| ϱ    | `\u{03F1}` | GREEK RHO SYMBOL                                    |
| ϲ    | `\u{03F2}` | GREEK LUNATE SIGMA SYMBOL                           |
| ϳ    | `\u{03F3}` | GREEK LETTER YOT                                    |
| ϴ    | `\u{03F4}` | GREEK CAPITAL THETA SYMBOL                          |
| ϵ    | `\u{03F5}` | GREEK LUNATE EPSILON SYMBOL                         |
| ϶    | `\u{03F6}` | GREEK REVERSED LUNATE EPSILON SYMBOL                |
| Ϸ    | `\u{03F7}` | GREEK CAPITAL LETTER SHO                            |
| ϸ    | `\u{03F8}` | GREEK SMALL LETTER SHO                              |
| Ϲ    | `\u{03F9}` | GREEK CAPITAL LUNATE SIGMA SYMBOL                   |
| Ϻ    | `\u{03FA}` | GREEK CAPITAL LETTER SAN                            |
| ϻ    | `\u{03FB}` | GREEK SMALL LETTER SAN                              |
| ϼ    | `\u{03FC}` | GREEK RHO WITH STROKE SYMBOL                        |
| Ͻ    | `\u{03FD}` | GREEK CAPITAL REVERSED LUNATE SIGMA SYMBOL          |
| Ͼ    | `\u{03FE}` | GREEK CAPITAL DOTTED LUNATE SIGMA SYMBOL            |
| Ͽ    | `\u{03FF}` | GREEK CAPITAL REVERSED DOTTED LUNATE SIGMA SYMBOL   |

## Letterlike Symbols (80)

| Char | Escape     | Name                           |
|------|------------|--------------------------------|
| ℀    | `\u{2100}` | ACCOUNT OF                     |
| ℁    | `\u{2101}` | ADDRESSED TO THE SUBJECT       |
| ℂ    | `\u{2102}` | DOUBLE-STRUCK CAPITAL C        |
| ℃    | `\u{2103}` | DEGREE CELSIUS                 |
| ℄    | `\u{2104}` | CENTRE LINE SYMBOL             |
| ℅    | `\u{2105}` | CARE OF                        |
| ℆    | `\u{2106}` | CADA UNA                       |
| ℇ    | `\u{2107}` | EULER CONSTANT                 |
| ℈    | `\u{2108}` | SCRUPLE                        |
| ℉    | `\u{2109}` | DEGREE FAHRENHEIT              |
| ℊ    | `\u{210A}` | SCRIPT SMALL G                 |
| ℋ    | `\u{210B}` | SCRIPT CAPITAL H               |
| ℌ    | `\u{210C}` | BLACK-LETTER CAPITAL H         |
| ℍ    | `\u{210D}` | DOUBLE-STRUCK CAPITAL H        |
| ℎ    | `\u{210E}` | PLANCK CONSTANT                |
| ℏ    | `\u{210F}` | PLANCK CONSTANT OVER TWO PI    |
| ℐ    | `\u{2110}` | SCRIPT CAPITAL I               |
| ℑ    | `\u{2111}` | BLACK-LETTER CAPITAL I         |
| ℒ    | `\u{2112}` | SCRIPT CAPITAL L               |
| ℓ    | `\u{2113}` | SCRIPT SMALL L                 |
| ℔    | `\u{2114}` | L B BAR SYMBOL                 |
| ℕ    | `\u{2115}` | DOUBLE-STRUCK CAPITAL N        |
| №    | `\u{2116}` | NUMERO SIGN                    |
| ℗    | `\u{2117}` | SOUND RECORDING COPYRIGHT      |
| ℘    | `\u{2118}` | SCRIPT CAPITAL P               |
| ℙ    | `\u{2119}` | DOUBLE-STRUCK CAPITAL P        |
| ℚ    | `\u{211A}` | DOUBLE-STRUCK CAPITAL Q        |
| ℛ    | `\u{211B}` | SCRIPT CAPITAL R               |
| ℜ    | `\u{211C}` | BLACK-LETTER CAPITAL R         |
| ℝ    | `\u{211D}` | DOUBLE-STRUCK CAPITAL R        |
| ℞    | `\u{211E}` | PRESCRIPTION TAKE              |
| ℟    | `\u{211F}` | RESPONSE                       |
| ℠    | `\u{2120}` | SERVICE MARK                   |
| ℡    | `\u{2121}` | TELEPHONE SIGN                 |
| ™    | `\u{2122}` | TRADE MARK SIGN                |
| ℣    | `\u{2123}` | VERSICLE                       |
| ℤ    | `\u{2124}` | DOUBLE-STRUCK CAPITAL Z        |
| ℥    | `\u{2125}` | OUNCE SIGN                     |
| Ω    | `\u{2126}` | OHM SIGN                       |
| ℧    | `\u{2127}` | INVERTED OHM SIGN              |
| ℨ    | `\u{2128}` | BLACK-LETTER CAPITAL Z         |
| ℩    | `\u{2129}` | TURNED GREEK SMALL LETTER IOTA |
| K    | `\u{212A}` | KELVIN SIGN                    |
| Å    | `\u{212B}` | ANGSTROM SIGN                  |
| ℬ    | `\u{212C}` | SCRIPT CAPITAL B               |
| ℭ    | `\u{212D}` | BLACK-LETTER CAPITAL C         |
| ℮    | `\u{212E}` | ESTIMATED SYMBOL               |
| ℯ    | `\u{212F}` | SCRIPT SMALL E                 |
| ℰ    | `\u{2130}` | SCRIPT CAPITAL E               |
| ℱ    | `\u{2131}` | SCRIPT CAPITAL F               |
| Ⅎ    | `\u{2132}` | TURNED CAPITAL F               |
| ℳ    | `\u{2133}` | SCRIPT CAPITAL M               |
| ℴ    | `\u{2134}` | SCRIPT SMALL O                 |
| ℵ    | `\u{2135}` | ALEF SYMBOL                    |
| ℶ    | `\u{2136}` | BET SYMBOL                     |
| ℷ    | `\u{2137}` | GIMEL SYMBOL                   |
| ℸ    | `\u{2138}` | DALET SYMBOL                   |
| ℹ    | `\u{2139}` | INFORMATION SOURCE             |
| ℺    | `\u{213A}` | ROTATED CAPITAL Q              |
| ℻    | `\u{213B}` | FACSIMILE SIGN                 |
| ℼ    | `\u{213C}` | DOUBLE-STRUCK SMALL PI         |
| ℽ    | `\u{213D}` | DOUBLE-STRUCK SMALL GAMMA      |
| ℾ    | `\u{213E}` | DOUBLE-STRUCK CAPITAL GAMMA    |
| ℿ    | `\u{213F}` | DOUBLE-STRUCK CAPITAL PI       |
| ⅀    | `\u{2140}` | DOUBLE-STRUCK N-ARY SUMMATION  |
| ⅁    | `\u{2141}` | TURNED SANS-SERIF CAPITAL G    |
| ⅂    | `\u{2142}` | TURNED SANS-SERIF CAPITAL L    |
| ⅃    | `\u{2143}` | REVERSED SANS-SERIF CAPITAL L  |
| ⅄    | `\u{2144}` | TURNED SANS-SERIF CAPITAL Y    |
| ⅅ    | `\u{2145}` | DOUBLE-STRUCK ITALIC CAPITAL D |
| ⅆ    | `\u{2146}` | DOUBLE-STRUCK ITALIC SMALL D   |
| ⅇ    | `\u{2147}` | DOUBLE-STRUCK ITALIC SMALL E   |
| ⅈ    | `\u{2148}` | DOUBLE-STRUCK ITALIC SMALL I   |
| ⅉ    | `\u{2149}` | DOUBLE-STRUCK ITALIC SMALL J   |
| ⅊    | `\u{214A}` | PROPERTY LINE                  |
| ⅋    | `\u{214B}` | TURNED AMPERSAND               |
| ⅌    | `\u{214C}` | PER SIGN                       |
| ⅍    | `\u{214D}` | AKTIESELSKAB                   |
| ⅎ    | `\u{214E}` | TURNED SMALL F                 |
| ⅏    | `\u{214F}` | SYMBOL FOR SAMARITAN SOURCE    |

## Braille Patterns (256)

| Char | Escape     | Name                          |
|------|------------|-------------------------------|
| ⠀    | `\u{2800}` | BRAILLE PATTERN BLANK         |
| ⠁    | `\u{2801}` | BRAILLE PATTERN DOTS-1        |
| ⠂    | `\u{2802}` | BRAILLE PATTERN DOTS-2        |
| ⠃    | `\u{2803}` | BRAILLE PATTERN DOTS-12       |
| ⠄    | `\u{2804}` | BRAILLE PATTERN DOTS-3        |
| ⠅    | `\u{2805}` | BRAILLE PATTERN DOTS-13       |
| ⠆    | `\u{2806}` | BRAILLE PATTERN DOTS-23       |
| ⠇    | `\u{2807}` | BRAILLE PATTERN DOTS-123      |
| ⠈    | `\u{2808}` | BRAILLE PATTERN DOTS-4        |
| ⠉    | `\u{2809}` | BRAILLE PATTERN DOTS-14       |
| ⠊    | `\u{280A}` | BRAILLE PATTERN DOTS-24       |
| ⠋    | `\u{280B}` | BRAILLE PATTERN DOTS-124      |
| ⠌    | `\u{280C}` | BRAILLE PATTERN DOTS-34       |
| ⠍    | `\u{280D}` | BRAILLE PATTERN DOTS-134      |
| ⠎    | `\u{280E}` | BRAILLE PATTERN DOTS-234      |
| ⠏    | `\u{280F}` | BRAILLE PATTERN DOTS-1234     |
| ⠐    | `\u{2810}` | BRAILLE PATTERN DOTS-5        |
| ⠑    | `\u{2811}` | BRAILLE PATTERN DOTS-15       |
| ⠒    | `\u{2812}` | BRAILLE PATTERN DOTS-25       |
| ⠓    | `\u{2813}` | BRAILLE PATTERN DOTS-125      |
| ⠔    | `\u{2814}` | BRAILLE PATTERN DOTS-35       |
| ⠕    | `\u{2815}` | BRAILLE PATTERN DOTS-135      |
| ⠖    | `\u{2816}` | BRAILLE PATTERN DOTS-235      |
| ⠗    | `\u{2817}` | BRAILLE PATTERN DOTS-1235     |
| ⠘    | `\u{2818}` | BRAILLE PATTERN DOTS-45       |
| ⠙    | `\u{2819}` | BRAILLE PATTERN DOTS-145      |
| ⠚    | `\u{281A}` | BRAILLE PATTERN DOTS-245      |
| ⠛    | `\u{281B}` | BRAILLE PATTERN DOTS-1245     |
| ⠜    | `\u{281C}` | BRAILLE PATTERN DOTS-345      |
| ⠝    | `\u{281D}` | BRAILLE PATTERN DOTS-1345     |
| ⠞    | `\u{281E}` | BRAILLE PATTERN DOTS-2345     |
| ⠟    | `\u{281F}` | BRAILLE PATTERN DOTS-12345    |
| ⠠    | `\u{2820}` | BRAILLE PATTERN DOTS-6        |
| ⠡    | `\u{2821}` | BRAILLE PATTERN DOTS-16       |
| ⠢    | `\u{2822}` | BRAILLE PATTERN DOTS-26       |
| ⠣    | `\u{2823}` | BRAILLE PATTERN DOTS-126      |
| ⠤    | `\u{2824}` | BRAILLE PATTERN DOTS-36       |
| ⠥    | `\u{2825}` | BRAILLE PATTERN DOTS-136      |
| ⠦    | `\u{2826}` | BRAILLE PATTERN DOTS-236      |
| ⠧    | `\u{2827}` | BRAILLE PATTERN DOTS-1236     |
| ⠨    | `\u{2828}` | BRAILLE PATTERN DOTS-46       |
| ⠩    | `\u{2829}` | BRAILLE PATTERN DOTS-146      |
| ⠪    | `\u{282A}` | BRAILLE PATTERN DOTS-246      |
| ⠫    | `\u{282B}` | BRAILLE PATTERN DOTS-1246     |
| ⠬    | `\u{282C}` | BRAILLE PATTERN DOTS-346      |
| ⠭    | `\u{282D}` | BRAILLE PATTERN DOTS-1346     |
| ⠮    | `\u{282E}` | BRAILLE PATTERN DOTS-2346     |
| ⠯    | `\u{282F}` | BRAILLE PATTERN DOTS-12346    |
| ⠰    | `\u{2830}` | BRAILLE PATTERN DOTS-56       |
| ⠱    | `\u{2831}` | BRAILLE PATTERN DOTS-156      |
| ⠲    | `\u{2832}` | BRAILLE PATTERN DOTS-256      |
| ⠳    | `\u{2833}` | BRAILLE PATTERN DOTS-1256     |
| ⠴    | `\u{2834}` | BRAILLE PATTERN DOTS-356      |
| ⠵    | `\u{2835}` | BRAILLE PATTERN DOTS-1356     |
| ⠶    | `\u{2836}` | BRAILLE PATTERN DOTS-2356     |
| ⠷    | `\u{2837}` | BRAILLE PATTERN DOTS-12356    |
| ⠸    | `\u{2838}` | BRAILLE PATTERN DOTS-456      |
| ⠹    | `\u{2839}` | BRAILLE PATTERN DOTS-1456     |
| ⠺    | `\u{283A}` | BRAILLE PATTERN DOTS-2456     |
| ⠻    | `\u{283B}` | BRAILLE PATTERN DOTS-12456    |
| ⠼    | `\u{283C}` | BRAILLE PATTERN DOTS-3456     |
| ⠽    | `\u{283D}` | BRAILLE PATTERN DOTS-13456    |
| ⠾    | `\u{283E}` | BRAILLE PATTERN DOTS-23456    |
| ⠿    | `\u{283F}` | BRAILLE PATTERN DOTS-123456   |
| ⡀    | `\u{2840}` | BRAILLE PATTERN DOTS-7        |
| ⡁    | `\u{2841}` | BRAILLE PATTERN DOTS-17       |
| ⡂    | `\u{2842}` | BRAILLE PATTERN DOTS-27       |
| ⡃    | `\u{2843}` | BRAILLE PATTERN DOTS-127      |
| ⡄    | `\u{2844}` | BRAILLE PATTERN DOTS-37       |
| ⡅    | `\u{2845}` | BRAILLE PATTERN DOTS-137      |
| ⡆    | `\u{2846}` | BRAILLE PATTERN DOTS-237      |
| ⡇    | `\u{2847}` | BRAILLE PATTERN DOTS-1237     |
| ⡈    | `\u{2848}` | BRAILLE PATTERN DOTS-47       |
| ⡉    | `\u{2849}` | BRAILLE PATTERN DOTS-147      |
| ⡊    | `\u{284A}` | BRAILLE PATTERN DOTS-247      |
| ⡋    | `\u{284B}` | BRAILLE PATTERN DOTS-1247     |
| ⡌    | `\u{284C}` | BRAILLE PATTERN DOTS-347      |
| ⡍    | `\u{284D}` | BRAILLE PATTERN DOTS-1347     |
| ⡎    | `\u{284E}` | BRAILLE PATTERN DOTS-2347     |
| ⡏    | `\u{284F}` | BRAILLE PATTERN DOTS-12347    |
| ⡐    | `\u{2850}` | BRAILLE PATTERN DOTS-57       |
| ⡑    | `\u{2851}` | BRAILLE PATTERN DOTS-157      |
| ⡒    | `\u{2852}` | BRAILLE PATTERN DOTS-257      |
| ⡓    | `\u{2853}` | BRAILLE PATTERN DOTS-1257     |
| ⡔    | `\u{2854}` | BRAILLE PATTERN DOTS-357      |
| ⡕    | `\u{2855}` | BRAILLE PATTERN DOTS-1357     |
| ⡖    | `\u{2856}` | BRAILLE PATTERN DOTS-2357     |
| ⡗    | `\u{2857}` | BRAILLE PATTERN DOTS-12357    |
| ⡘    | `\u{2858}` | BRAILLE PATTERN DOTS-457      |
| ⡙    | `\u{2859}` | BRAILLE PATTERN DOTS-1457     |
| ⡚    | `\u{285A}` | BRAILLE PATTERN DOTS-2457     |
| ⡛    | `\u{285B}` | BRAILLE PATTERN DOTS-12457    |
| ⡜    | `\u{285C}` | BRAILLE PATTERN DOTS-3457     |
| ⡝    | `\u{285D}` | BRAILLE PATTERN DOTS-13457    |
| ⡞    | `\u{285E}` | BRAILLE PATTERN DOTS-23457    |
| ⡟    | `\u{285F}` | BRAILLE PATTERN DOTS-123457   |
| ⡠    | `\u{2860}` | BRAILLE PATTERN DOTS-67       |
| ⡡    | `\u{2861}` | BRAILLE PATTERN DOTS-167      |
| ⡢    | `\u{2862}` | BRAILLE PATTERN DOTS-267      |
| ⡣    | `\u{2863}` | BRAILLE PATTERN DOTS-1267     |
| ⡤    | `\u{2864}` | BRAILLE PATTERN DOTS-367      |
| ⡥    | `\u{2865}` | BRAILLE PATTERN DOTS-1367     |
| ⡦    | `\u{2866}` | BRAILLE PATTERN DOTS-2367     |
| ⡧    | `\u{2867}` | BRAILLE PATTERN DOTS-12367    |
| ⡨    | `\u{2868}` | BRAILLE PATTERN DOTS-467      |
| ⡩    | `\u{2869}` | BRAILLE PATTERN DOTS-1467     |
| ⡪    | `\u{286A}` | BRAILLE PATTERN DOTS-2467     |
| ⡫    | `\u{286B}` | BRAILLE PATTERN DOTS-12467    |
| ⡬    | `\u{286C}` | BRAILLE PATTERN DOTS-3467     |
| ⡭    | `\u{286D}` | BRAILLE PATTERN DOTS-13467    |
| ⡮    | `\u{286E}` | BRAILLE PATTERN DOTS-23467    |
| ⡯    | `\u{286F}` | BRAILLE PATTERN DOTS-123467   |
| ⡰    | `\u{2870}` | BRAILLE PATTERN DOTS-567      |
| ⡱    | `\u{2871}` | BRAILLE PATTERN DOTS-1567     |
| ⡲    | `\u{2872}` | BRAILLE PATTERN DOTS-2567     |
| ⡳    | `\u{2873}` | BRAILLE PATTERN DOTS-12567    |
| ⡴    | `\u{2874}` | BRAILLE PATTERN DOTS-3567     |
| ⡵    | `\u{2875}` | BRAILLE PATTERN DOTS-13567    |
| ⡶    | `\u{2876}` | BRAILLE PATTERN DOTS-23567    |
| ⡷    | `\u{2877}` | BRAILLE PATTERN DOTS-123567   |
| ⡸    | `\u{2878}` | BRAILLE PATTERN DOTS-4567     |
| ⡹    | `\u{2879}` | BRAILLE PATTERN DOTS-14567    |
| ⡺    | `\u{287A}` | BRAILLE PATTERN DOTS-24567    |
| ⡻    | `\u{287B}` | BRAILLE PATTERN DOTS-124567   |
| ⡼    | `\u{287C}` | BRAILLE PATTERN DOTS-34567    |
| ⡽    | `\u{287D}` | BRAILLE PATTERN DOTS-134567   |
| ⡾    | `\u{287E}` | BRAILLE PATTERN DOTS-234567   |
| ⡿    | `\u{287F}` | BRAILLE PATTERN DOTS-1234567  |
| ⢀    | `\u{2880}` | BRAILLE PATTERN DOTS-8        |
| ⢁    | `\u{2881}` | BRAILLE PATTERN DOTS-18       |
| ⢂    | `\u{2882}` | BRAILLE PATTERN DOTS-28       |
| ⢃    | `\u{2883}` | BRAILLE PATTERN DOTS-128      |
| ⢄    | `\u{2884}` | BRAILLE PATTERN DOTS-38       |
| ⢅    | `\u{2885}` | BRAILLE PATTERN DOTS-138      |
| ⢆    | `\u{2886}` | BRAILLE PATTERN DOTS-238      |
| ⢇    | `\u{2887}` | BRAILLE PATTERN DOTS-1238     |
| ⢈    | `\u{2888}` | BRAILLE PATTERN DOTS-48       |
| ⢉    | `\u{2889}` | BRAILLE PATTERN DOTS-148      |
| ⢊    | `\u{288A}` | BRAILLE PATTERN DOTS-248      |
| ⢋    | `\u{288B}` | BRAILLE PATTERN DOTS-1248     |
| ⢌    | `\u{288C}` | BRAILLE PATTERN DOTS-348      |
| ⢍    | `\u{288D}` | BRAILLE PATTERN DOTS-1348     |
| ⢎    | `\u{288E}` | BRAILLE PATTERN DOTS-2348     |
| ⢏    | `\u{288F}` | BRAILLE PATTERN DOTS-12348    |
| ⢐    | `\u{2890}` | BRAILLE PATTERN DOTS-58       |
| ⢑    | `\u{2891}` | BRAILLE PATTERN DOTS-158      |
| ⢒    | `\u{2892}` | BRAILLE PATTERN DOTS-258      |
| ⢓    | `\u{2893}` | BRAILLE PATTERN DOTS-1258     |
| ⢔    | `\u{2894}` | BRAILLE PATTERN DOTS-358      |
| ⢕    | `\u{2895}` | BRAILLE PATTERN DOTS-1358     |
| ⢖    | `\u{2896}` | BRAILLE PATTERN DOTS-2358     |
| ⢗    | `\u{2897}` | BRAILLE PATTERN DOTS-12358    |
| ⢘    | `\u{2898}` | BRAILLE PATTERN DOTS-458      |
| ⢙    | `\u{2899}` | BRAILLE PATTERN DOTS-1458     |
| ⢚    | `\u{289A}` | BRAILLE PATTERN DOTS-2458     |
| ⢛    | `\u{289B}` | BRAILLE PATTERN DOTS-12458    |
| ⢜    | `\u{289C}` | BRAILLE PATTERN DOTS-3458     |
| ⢝    | `\u{289D}` | BRAILLE PATTERN DOTS-13458    |
| ⢞    | `\u{289E}` | BRAILLE PATTERN DOTS-23458    |
| ⢟    | `\u{289F}` | BRAILLE PATTERN DOTS-123458   |
| ⢠    | `\u{28A0}` | BRAILLE PATTERN DOTS-68       |
| ⢡    | `\u{28A1}` | BRAILLE PATTERN DOTS-168      |
| ⢢    | `\u{28A2}` | BRAILLE PATTERN DOTS-268      |
| ⢣    | `\u{28A3}` | BRAILLE PATTERN DOTS-1268     |
| ⢤    | `\u{28A4}` | BRAILLE PATTERN DOTS-368      |
| ⢥    | `\u{28A5}` | BRAILLE PATTERN DOTS-1368     |
| ⢦    | `\u{28A6}` | BRAILLE PATTERN DOTS-2368     |
| ⢧    | `\u{28A7}` | BRAILLE PATTERN DOTS-12368    |
| ⢨    | `\u{28A8}` | BRAILLE PATTERN DOTS-468      |
| ⢩    | `\u{28A9}` | BRAILLE PATTERN DOTS-1468     |
| ⢪    | `\u{28AA}` | BRAILLE PATTERN DOTS-2468     |
| ⢫    | `\u{28AB}` | BRAILLE PATTERN DOTS-12468    |
| ⢬    | `\u{28AC}` | BRAILLE PATTERN DOTS-3468     |
| ⢭    | `\u{28AD}` | BRAILLE PATTERN DOTS-13468    |
| ⢮    | `\u{28AE}` | BRAILLE PATTERN DOTS-23468    |
| ⢯    | `\u{28AF}` | BRAILLE PATTERN DOTS-123468   |
| ⢰    | `\u{28B0}` | BRAILLE PATTERN DOTS-568      |
| ⢱    | `\u{28B1}` | BRAILLE PATTERN DOTS-1568     |
| ⢲    | `\u{28B2}` | BRAILLE PATTERN DOTS-2568     |
| ⢳    | `\u{28B3}` | BRAILLE PATTERN DOTS-12568    |
| ⢴    | `\u{28B4}` | BRAILLE PATTERN DOTS-3568     |
| ⢵    | `\u{28B5}` | BRAILLE PATTERN DOTS-13568    |
| ⢶    | `\u{28B6}` | BRAILLE PATTERN DOTS-23568    |
| ⢷    | `\u{28B7}` | BRAILLE PATTERN DOTS-123568   |
| ⢸    | `\u{28B8}` | BRAILLE PATTERN DOTS-4568     |
| ⢹    | `\u{28B9}` | BRAILLE PATTERN DOTS-14568    |
| ⢺    | `\u{28BA}` | BRAILLE PATTERN DOTS-24568    |
| ⢻    | `\u{28BB}` | BRAILLE PATTERN DOTS-124568   |
| ⢼    | `\u{28BC}` | BRAILLE PATTERN DOTS-34568    |
| ⢽    | `\u{28BD}` | BRAILLE PATTERN DOTS-134568   |
| ⢾    | `\u{28BE}` | BRAILLE PATTERN DOTS-234568   |
| ⢿    | `\u{28BF}` | BRAILLE PATTERN DOTS-1234568  |
| ⣀    | `\u{28C0}` | BRAILLE PATTERN DOTS-78       |
| ⣁    | `\u{28C1}` | BRAILLE PATTERN DOTS-178      |
| ⣂    | `\u{28C2}` | BRAILLE PATTERN DOTS-278      |
| ⣃    | `\u{28C3}` | BRAILLE PATTERN DOTS-1278     |
| ⣄    | `\u{28C4}` | BRAILLE PATTERN DOTS-378      |
| ⣅    | `\u{28C5}` | BRAILLE PATTERN DOTS-1378     |
| ⣆    | `\u{28C6}` | BRAILLE PATTERN DOTS-2378     |
| ⣇    | `\u{28C7}` | BRAILLE PATTERN DOTS-12378    |
| ⣈    | `\u{28C8}` | BRAILLE PATTERN DOTS-478      |
| ⣉    | `\u{28C9}` | BRAILLE PATTERN DOTS-1478     |
| ⣊    | `\u{28CA}` | BRAILLE PATTERN DOTS-2478     |
| ⣋    | `\u{28CB}` | BRAILLE PATTERN DOTS-12478    |
| ⣌    | `\u{28CC}` | BRAILLE PATTERN DOTS-3478     |
| ⣍    | `\u{28CD}` | BRAILLE PATTERN DOTS-13478    |
| ⣎    | `\u{28CE}` | BRAILLE PATTERN DOTS-23478    |
| ⣏    | `\u{28CF}` | BRAILLE PATTERN DOTS-123478   |
| ⣐    | `\u{28D0}` | BRAILLE PATTERN DOTS-578      |
| ⣑    | `\u{28D1}` | BRAILLE PATTERN DOTS-1578     |
| ⣒    | `\u{28D2}` | BRAILLE PATTERN DOTS-2578     |
| ⣓    | `\u{28D3}` | BRAILLE PATTERN DOTS-12578    |
| ⣔    | `\u{28D4}` | BRAILLE PATTERN DOTS-3578     |
| ⣕    | `\u{28D5}` | BRAILLE PATTERN DOTS-13578    |
| ⣖    | `\u{28D6}` | BRAILLE PATTERN DOTS-23578    |
| ⣗    | `\u{28D7}` | BRAILLE PATTERN DOTS-123578   |
| ⣘    | `\u{28D8}` | BRAILLE PATTERN DOTS-4578     |
| ⣙    | `\u{28D9}` | BRAILLE PATTERN DOTS-14578    |
| ⣚    | `\u{28DA}` | BRAILLE PATTERN DOTS-24578    |
| ⣛    | `\u{28DB}` | BRAILLE PATTERN DOTS-124578   |
| ⣜    | `\u{28DC}` | BRAILLE PATTERN DOTS-34578    |
| ⣝    | `\u{28DD}` | BRAILLE PATTERN DOTS-134578   |
| ⣞    | `\u{28DE}` | BRAILLE PATTERN DOTS-234578   |
| ⣟    | `\u{28DF}` | BRAILLE PATTERN DOTS-1234578  |
| ⣠    | `\u{28E0}` | BRAILLE PATTERN DOTS-678      |
| ⣡    | `\u{28E1}` | BRAILLE PATTERN DOTS-1678     |
| ⣢    | `\u{28E2}` | BRAILLE PATTERN DOTS-2678     |
| ⣣    | `\u{28E3}` | BRAILLE PATTERN DOTS-12678    |
| ⣤    | `\u{28E4}` | BRAILLE PATTERN DOTS-3678     |
| ⣥    | `\u{28E5}` | BRAILLE PATTERN DOTS-13678    |
| ⣦    | `\u{28E6}` | BRAILLE PATTERN DOTS-23678    |
| ⣧    | `\u{28E7}` | BRAILLE PATTERN DOTS-123678   |
| ⣨    | `\u{28E8}` | BRAILLE PATTERN DOTS-4678     |
| ⣩    | `\u{28E9}` | BRAILLE PATTERN DOTS-14678    |
| ⣪    | `\u{28EA}` | BRAILLE PATTERN DOTS-24678    |
| ⣫    | `\u{28EB}` | BRAILLE PATTERN DOTS-124678   |
| ⣬    | `\u{28EC}` | BRAILLE PATTERN DOTS-34678    |
| ⣭    | `\u{28ED}` | BRAILLE PATTERN DOTS-134678   |
| ⣮    | `\u{28EE}` | BRAILLE PATTERN DOTS-234678   |
| ⣯    | `\u{28EF}` | BRAILLE PATTERN DOTS-1234678  |
| ⣰    | `\u{28F0}` | BRAILLE PATTERN DOTS-5678     |
| ⣱    | `\u{28F1}` | BRAILLE PATTERN DOTS-15678    |
| ⣲    | `\u{28F2}` | BRAILLE PATTERN DOTS-25678    |
| ⣳    | `\u{28F3}` | BRAILLE PATTERN DOTS-125678   |
| ⣴    | `\u{28F4}` | BRAILLE PATTERN DOTS-35678    |
| ⣵    | `\u{28F5}` | BRAILLE PATTERN DOTS-135678   |
| ⣶    | `\u{28F6}` | BRAILLE PATTERN DOTS-235678   |
| ⣷    | `\u{28F7}` | BRAILLE PATTERN DOTS-1235678  |
| ⣸    | `\u{28F8}` | BRAILLE PATTERN DOTS-45678    |
| ⣹    | `\u{28F9}` | BRAILLE PATTERN DOTS-145678   |
| ⣺    | `\u{28FA}` | BRAILLE PATTERN DOTS-245678   |
| ⣻    | `\u{28FB}` | BRAILLE PATTERN DOTS-1245678  |
| ⣼    | `\u{28FC}` | BRAILLE PATTERN DOTS-345678   |
| ⣽    | `\u{28FD}` | BRAILLE PATTERN DOTS-1345678  |
| ⣾    | `\u{28FE}` | BRAILLE PATTERN DOTS-2345678  |
| ⣿    | `\u{28FF}` | BRAILLE PATTERN DOTS-12345678 |

## Miscellaneous Technical (256)

| Char | Escape     | Name                                                     |
|------|------------|----------------------------------------------------------|
| ⌀    | `\u{2300}` | DIAMETER SIGN                                            |
| ⌁    | `\u{2301}` | ELECTRIC ARROW                                           |
| ⌂    | `\u{2302}` | HOUSE                                                    |
| ⌃    | `\u{2303}` | UP ARROWHEAD                                             |
| ⌄    | `\u{2304}` | DOWN ARROWHEAD                                           |
| ⌅    | `\u{2305}` | PROJECTIVE                                               |
| ⌆    | `\u{2306}` | PERSPECTIVE                                              |
| ⌇    | `\u{2307}` | WAVY LINE                                                |
| ⌈    | `\u{2308}` | LEFT CEILING                                             |
| ⌉    | `\u{2309}` | RIGHT CEILING                                            |
| ⌊    | `\u{230A}` | LEFT FLOOR                                               |
| ⌋    | `\u{230B}` | RIGHT FLOOR                                              |
| ⌌    | `\u{230C}` | BOTTOM RIGHT CROP                                        |
| ⌍    | `\u{230D}` | BOTTOM LEFT CROP                                         |
| ⌎    | `\u{230E}` | TOP RIGHT CROP                                           |
| ⌏    | `\u{230F}` | TOP LEFT CROP                                            |
| ⌐    | `\u{2310}` | REVERSED NOT SIGN                                        |
| ⌑    | `\u{2311}` | SQUARE LOZENGE                                           |
| ⌒    | `\u{2312}` | ARC                                                      |
| ⌓    | `\u{2313}` | SEGMENT                                                  |
| ⌔    | `\u{2314}` | SECTOR                                                   |
| ⌕    | `\u{2315}` | TELEPHONE RECORDER                                       |
| ⌖    | `\u{2316}` | POSITION INDICATOR                                       |
| ⌗    | `\u{2317}` | VIEWDATA SQUARE                                          |
| ⌘    | `\u{2318}` | PLACE OF INTEREST SIGN                                   |
| ⌙    | `\u{2319}` | TURNED NOT SIGN                                          |
| ⌚   | `\u{231A}` | WATCH                                                    |
| ⌛   | `\u{231B}` | HOURGLASS                                                |
| ⌜    | `\u{231C}` | TOP LEFT CORNER                                          |
| ⌝    | `\u{231D}` | TOP RIGHT CORNER                                         |
| ⌞    | `\u{231E}` | BOTTOM LEFT CORNER                                       |
| ⌟    | `\u{231F}` | BOTTOM RIGHT CORNER                                      |
| ⌠    | `\u{2320}` | TOP HALF INTEGRAL                                        |
| ⌡    | `\u{2321}` | BOTTOM HALF INTEGRAL                                     |
| ⌢    | `\u{2322}` | FROWN                                                    |
| ⌣    | `\u{2323}` | SMILE                                                    |
| ⌤    | `\u{2324}` | UP ARROWHEAD BETWEEN TWO HORIZONTAL BARS                 |
| ⌥    | `\u{2325}` | OPTION KEY                                               |
| ⌦    | `\u{2326}` | ERASE TO THE RIGHT                                       |
| ⌧    | `\u{2327}` | X IN A RECTANGLE BOX                                     |
| ⌨    | `\u{2328}` | KEYBOARD                                                 |
| 〈   | `\u{2329}` | LEFT-POINTING ANGLE BRACKET                              |
| 〉   | `\u{232A}` | RIGHT-POINTING ANGLE BRACKET                             |
| ⌫    | `\u{232B}` | ERASE TO THE LEFT                                        |
| ⌬    | `\u{232C}` | BENZENE RING                                             |
| ⌭    | `\u{232D}` | CYLINDRICITY                                             |
| ⌮    | `\u{232E}` | ALL AROUND-PROFILE                                       |
| ⌯    | `\u{232F}` | SYMMETRY                                                 |
| ⌰    | `\u{2330}` | TOTAL RUNOUT                                             |
| ⌱    | `\u{2331}` | DIMENSION ORIGIN                                         |
| ⌲    | `\u{2332}` | CONICAL TAPER                                            |
| ⌳    | `\u{2333}` | SLOPE                                                    |
| ⌴    | `\u{2334}` | COUNTERBORE                                              |
| ⌵    | `\u{2335}` | COUNTERSINK                                              |
| ⌶    | `\u{2336}` | APL FUNCTIONAL SYMBOL I-BEAM                             |
| ⌷    | `\u{2337}` | APL FUNCTIONAL SYMBOL SQUISH QUAD                        |
| ⌸    | `\u{2338}` | APL FUNCTIONAL SYMBOL QUAD EQUAL                         |
| ⌹    | `\u{2339}` | APL FUNCTIONAL SYMBOL QUAD DIVIDE                        |
| ⌺    | `\u{233A}` | APL FUNCTIONAL SYMBOL QUAD DIAMOND                       |
| ⌻    | `\u{233B}` | APL FUNCTIONAL SYMBOL QUAD JOT                           |
| ⌼    | `\u{233C}` | APL FUNCTIONAL SYMBOL QUAD CIRCLE                        |
| ⌽    | `\u{233D}` | APL FUNCTIONAL SYMBOL CIRCLE STILE                       |
| ⌾    | `\u{233E}` | APL FUNCTIONAL SYMBOL CIRCLE JOT                         |
| ⌿    | `\u{233F}` | APL FUNCTIONAL SYMBOL SLASH BAR                          |
| ⍀    | `\u{2340}` | APL FUNCTIONAL SYMBOL BACKSLASH BAR                      |
| ⍁    | `\u{2341}` | APL FUNCTIONAL SYMBOL QUAD SLASH                         |
| ⍂    | `\u{2342}` | APL FUNCTIONAL SYMBOL QUAD BACKSLASH                     |
| ⍃    | `\u{2343}` | APL FUNCTIONAL SYMBOL QUAD LESS-THAN                     |
| ⍄    | `\u{2344}` | APL FUNCTIONAL SYMBOL QUAD GREATER-THAN                  |
| ⍅    | `\u{2345}` | APL FUNCTIONAL SYMBOL LEFTWARDS VANE                     |
| ⍆    | `\u{2346}` | APL FUNCTIONAL SYMBOL RIGHTWARDS VANE                    |
| ⍇    | `\u{2347}` | APL FUNCTIONAL SYMBOL QUAD LEFTWARDS ARROW               |
| ⍈    | `\u{2348}` | APL FUNCTIONAL SYMBOL QUAD RIGHTWARDS ARROW              |
| ⍉    | `\u{2349}` | APL FUNCTIONAL SYMBOL CIRCLE BACKSLASH                   |
| ⍊    | `\u{234A}` | APL FUNCTIONAL SYMBOL DOWN TACK UNDERBAR                 |
| ⍋    | `\u{234B}` | APL FUNCTIONAL SYMBOL DELTA STILE                        |
| ⍌    | `\u{234C}` | APL FUNCTIONAL SYMBOL QUAD DOWN CARET                    |
| ⍍    | `\u{234D}` | APL FUNCTIONAL SYMBOL QUAD DELTA                         |
| ⍎    | `\u{234E}` | APL FUNCTIONAL SYMBOL DOWN TACK JOT                      |
| ⍏    | `\u{234F}` | APL FUNCTIONAL SYMBOL UPWARDS VANE                       |
| ⍐    | `\u{2350}` | APL FUNCTIONAL SYMBOL QUAD UPWARDS ARROW                 |
| ⍑    | `\u{2351}` | APL FUNCTIONAL SYMBOL UP TACK OVERBAR                    |
| ⍒    | `\u{2352}` | APL FUNCTIONAL SYMBOL DEL STILE                          |
| ⍓    | `\u{2353}` | APL FUNCTIONAL SYMBOL QUAD UP CARET                      |
| ⍔    | `\u{2354}` | APL FUNCTIONAL SYMBOL QUAD DEL                           |
| ⍕    | `\u{2355}` | APL FUNCTIONAL SYMBOL UP TACK JOT                        |
| ⍖    | `\u{2356}` | APL FUNCTIONAL SYMBOL DOWNWARDS VANE                     |
| ⍗    | `\u{2357}` | APL FUNCTIONAL SYMBOL QUAD DOWNWARDS ARROW               |
| ⍘    | `\u{2358}` | APL FUNCTIONAL SYMBOL QUOTE UNDERBAR                     |
| ⍙    | `\u{2359}` | APL FUNCTIONAL SYMBOL DELTA UNDERBAR                     |
| ⍚    | `\u{235A}` | APL FUNCTIONAL SYMBOL DIAMOND UNDERBAR                   |
| ⍛    | `\u{235B}` | APL FUNCTIONAL SYMBOL JOT UNDERBAR                       |
| ⍜    | `\u{235C}` | APL FUNCTIONAL SYMBOL CIRCLE UNDERBAR                    |
| ⍝    | `\u{235D}` | APL FUNCTIONAL SYMBOL UP SHOE JOT                        |
| ⍞    | `\u{235E}` | APL FUNCTIONAL SYMBOL QUOTE QUAD                         |
| ⍟    | `\u{235F}` | APL FUNCTIONAL SYMBOL CIRCLE STAR                        |
| ⍠    | `\u{2360}` | APL FUNCTIONAL SYMBOL QUAD COLON                         |
| ⍡    | `\u{2361}` | APL FUNCTIONAL SYMBOL UP TACK DIAERESIS                  |
| ⍢    | `\u{2362}` | APL FUNCTIONAL SYMBOL DEL DIAERESIS                      |
| ⍣    | `\u{2363}` | APL FUNCTIONAL SYMBOL STAR DIAERESIS                     |
| ⍤    | `\u{2364}` | APL FUNCTIONAL SYMBOL JOT DIAERESIS                      |
| ⍥    | `\u{2365}` | APL FUNCTIONAL SYMBOL CIRCLE DIAERESIS                   |
| ⍦    | `\u{2366}` | APL FUNCTIONAL SYMBOL DOWN SHOE STILE                    |
| ⍧    | `\u{2367}` | APL FUNCTIONAL SYMBOL LEFT SHOE STILE                    |
| ⍨    | `\u{2368}` | APL FUNCTIONAL SYMBOL TILDE DIAERESIS                    |
| ⍩    | `\u{2369}` | APL FUNCTIONAL SYMBOL GREATER-THAN DIAERESIS             |
| ⍪    | `\u{236A}` | APL FUNCTIONAL SYMBOL COMMA BAR                          |
| ⍫    | `\u{236B}` | APL FUNCTIONAL SYMBOL DEL TILDE                          |
| ⍬    | `\u{236C}` | APL FUNCTIONAL SYMBOL ZILDE                              |
| ⍭    | `\u{236D}` | APL FUNCTIONAL SYMBOL STILE TILDE                        |
| ⍮    | `\u{236E}` | APL FUNCTIONAL SYMBOL SEMICOLON UNDERBAR                 |
| ⍯    | `\u{236F}` | APL FUNCTIONAL SYMBOL QUAD NOT EQUAL                     |
| ⍰    | `\u{2370}` | APL FUNCTIONAL SYMBOL QUAD QUESTION                      |
| ⍱    | `\u{2371}` | APL FUNCTIONAL SYMBOL DOWN CARET TILDE                   |
| ⍲    | `\u{2372}` | APL FUNCTIONAL SYMBOL UP CARET TILDE                     |
| ⍳    | `\u{2373}` | APL FUNCTIONAL SYMBOL IOTA                               |
| ⍴    | `\u{2374}` | APL FUNCTIONAL SYMBOL RHO                                |
| ⍵    | `\u{2375}` | APL FUNCTIONAL SYMBOL OMEGA                              |
| ⍶    | `\u{2376}` | APL FUNCTIONAL SYMBOL ALPHA UNDERBAR                     |
| ⍷    | `\u{2377}` | APL FUNCTIONAL SYMBOL EPSILON UNDERBAR                   |
| ⍸    | `\u{2378}` | APL FUNCTIONAL SYMBOL IOTA UNDERBAR                      |
| ⍹    | `\u{2379}` | APL FUNCTIONAL SYMBOL OMEGA UNDERBAR                     |
| ⍺    | `\u{237A}` | APL FUNCTIONAL SYMBOL ALPHA                              |
| ⍻    | `\u{237B}` | NOT CHECK MARK                                           |
| ⍼    | `\u{237C}` | RIGHT ANGLE WITH DOWNWARDS ZIGZAG ARROW                  |
| ⍽    | `\u{237D}` | SHOULDERED OPEN BOX                                      |
| ⍾    | `\u{237E}` | BELL SYMBOL                                              |
| ⍿    | `\u{237F}` | VERTICAL LINE WITH MIDDLE DOT                            |
| ⎀    | `\u{2380}` | INSERTION SYMBOL                                         |
| ⎁    | `\u{2381}` | CONTINUOUS UNDERLINE SYMBOL                              |
| ⎂    | `\u{2382}` | DISCONTINUOUS UNDERLINE SYMBOL                           |
| ⎃    | `\u{2383}` | EMPHASIS SYMBOL                                          |
| ⎄    | `\u{2384}` | COMPOSITION SYMBOL                                       |
| ⎅    | `\u{2385}` | WHITE SQUARE WITH CENTRE VERTICAL LINE                   |
| ⎆    | `\u{2386}` | ENTER SYMBOL                                             |
| ⎇    | `\u{2387}` | ALTERNATIVE KEY SYMBOL                                   |
| ⎈    | `\u{2388}` | HELM SYMBOL                                              |
| ⎉    | `\u{2389}` | CIRCLED HORIZONTAL BAR WITH NOTCH                        |
| ⎊    | `\u{238A}` | CIRCLED TRIANGLE DOWN                                    |
| ⎋    | `\u{238B}` | BROKEN CIRCLE WITH NORTHWEST ARROW                       |
| ⎌    | `\u{238C}` | UNDO SYMBOL                                              |
| ⎍    | `\u{238D}` | MONOSTABLE SYMBOL                                        |
| ⎎    | `\u{238E}` | HYSTERESIS SYMBOL                                        |
| ⎏    | `\u{238F}` | OPEN-CIRCUIT-OUTPUT H-TYPE SYMBOL                        |
| ⎐    | `\u{2390}` | OPEN-CIRCUIT-OUTPUT L-TYPE SYMBOL                        |
| ⎑    | `\u{2391}` | PASSIVE-PULL-DOWN-OUTPUT SYMBOL                          |
| ⎒    | `\u{2392}` | PASSIVE-PULL-UP-OUTPUT SYMBOL                            |
| ⎓    | `\u{2393}` | DIRECT CURRENT SYMBOL FORM TWO                           |
| ⎔    | `\u{2394}` | SOFTWARE-FUNCTION SYMBOL                                 |
| ⎕    | `\u{2395}` | APL FUNCTIONAL SYMBOL QUAD                               |
| ⎖    | `\u{2396}` | DECIMAL SEPARATOR KEY SYMBOL                             |
| ⎗    | `\u{2397}` | PREVIOUS PAGE                                            |
| ⎘    | `\u{2398}` | NEXT PAGE                                                |
| ⎙    | `\u{2399}` | PRINT SCREEN SYMBOL                                      |
| ⎚    | `\u{239A}` | CLEAR SCREEN SYMBOL                                      |
| ⎛    | `\u{239B}` | LEFT PARENTHESIS UPPER HOOK                              |
| ⎜    | `\u{239C}` | LEFT PARENTHESIS EXTENSION                               |
| ⎝    | `\u{239D}` | LEFT PARENTHESIS LOWER HOOK                              |
| ⎞    | `\u{239E}` | RIGHT PARENTHESIS UPPER HOOK                             |
| ⎟    | `\u{239F}` | RIGHT PARENTHESIS EXTENSION                              |
| ⎠    | `\u{23A0}` | RIGHT PARENTHESIS LOWER HOOK                             |
| ⎡    | `\u{23A1}` | LEFT SQUARE BRACKET UPPER CORNER                         |
| ⎢    | `\u{23A2}` | LEFT SQUARE BRACKET EXTENSION                            |
| ⎣    | `\u{23A3}` | LEFT SQUARE BRACKET LOWER CORNER                         |
| ⎤    | `\u{23A4}` | RIGHT SQUARE BRACKET UPPER CORNER                        |
| ⎥    | `\u{23A5}` | RIGHT SQUARE BRACKET EXTENSION                           |
| ⎦    | `\u{23A6}` | RIGHT SQUARE BRACKET LOWER CORNER                        |
| ⎧    | `\u{23A7}` | LEFT CURLY BRACKET UPPER HOOK                            |
| ⎨    | `\u{23A8}` | LEFT CURLY BRACKET MIDDLE PIECE                          |
| ⎩    | `\u{23A9}` | LEFT CURLY BRACKET LOWER HOOK                            |
| ⎪    | `\u{23AA}` | CURLY BRACKET EXTENSION                                  |
| ⎫    | `\u{23AB}` | RIGHT CURLY BRACKET UPPER HOOK                           |
| ⎬    | `\u{23AC}` | RIGHT CURLY BRACKET MIDDLE PIECE                         |
| ⎭    | `\u{23AD}` | RIGHT CURLY BRACKET LOWER HOOK                           |
| ⎮    | `\u{23AE}` | INTEGRAL EXTENSION                                       |
| ⎯    | `\u{23AF}` | HORIZONTAL LINE EXTENSION                                |
| ⎰    | `\u{23B0}` | UPPER LEFT OR LOWER RIGHT CURLY BRACKET SECTION          |
| ⎱    | `\u{23B1}` | UPPER RIGHT OR LOWER LEFT CURLY BRACKET SECTION          |
| ⎲    | `\u{23B2}` | SUMMATION TOP                                            |
| ⎳    | `\u{23B3}` | SUMMATION BOTTOM                                         |
| ⎴    | `\u{23B4}` | TOP SQUARE BRACKET                                       |
| ⎵    | `\u{23B5}` | BOTTOM SQUARE BRACKET                                    |
| ⎶    | `\u{23B6}` | BOTTOM SQUARE BRACKET OVER TOP SQUARE BRACKET            |
| ⎷    | `\u{23B7}` | RADICAL SYMBOL BOTTOM                                    |
| ⎸    | `\u{23B8}` | LEFT VERTICAL BOX LINE                                   |
| ⎹    | `\u{23B9}` | RIGHT VERTICAL BOX LINE                                  |
| ⎺    | `\u{23BA}` | HORIZONTAL SCAN LINE-1                                   |
| ⎻    | `\u{23BB}` | HORIZONTAL SCAN LINE-3                                   |
| ⎼    | `\u{23BC}` | HORIZONTAL SCAN LINE-7                                   |
| ⎽    | `\u{23BD}` | HORIZONTAL SCAN LINE-9                                   |
| ⎾    | `\u{23BE}` | DENTISTRY SYMBOL LIGHT VERTICAL AND TOP RIGHT            |
| ⎿    | `\u{23BF}` | DENTISTRY SYMBOL LIGHT VERTICAL AND BOTTOM RIGHT         |
| ⏀    | `\u{23C0}` | DENTISTRY SYMBOL LIGHT VERTICAL WITH CIRCLE              |
| ⏁    | `\u{23C1}` | DENTISTRY SYMBOL LIGHT DOWN AND HORIZONTAL WITH CIRCLE   |
| ⏂    | `\u{23C2}` | DENTISTRY SYMBOL LIGHT UP AND HORIZONTAL WITH CIRCLE     |
| ⏃    | `\u{23C3}` | DENTISTRY SYMBOL LIGHT VERTICAL WITH TRIANGLE            |
| ⏄    | `\u{23C4}` | DENTISTRY SYMBOL LIGHT DOWN AND HORIZONTAL WITH TRIANGLE |
| ⏅    | `\u{23C5}` | DENTISTRY SYMBOL LIGHT UP AND HORIZONTAL WITH TRIANGLE   |
| ⏆    | `\u{23C6}` | DENTISTRY SYMBOL LIGHT VERTICAL AND WAVE                 |
| ⏇    | `\u{23C7}` | DENTISTRY SYMBOL LIGHT DOWN AND HORIZONTAL WITH WAVE     |
| ⏈    | `\u{23C8}` | DENTISTRY SYMBOL LIGHT UP AND HORIZONTAL WITH WAVE       |
| ⏉    | `\u{23C9}` | DENTISTRY SYMBOL LIGHT DOWN AND HORIZONTAL               |
| ⏊    | `\u{23CA}` | DENTISTRY SYMBOL LIGHT UP AND HORIZONTAL                 |
| ⏋    | `\u{23CB}` | DENTISTRY SYMBOL LIGHT VERTICAL AND TOP LEFT             |
| ⏌    | `\u{23CC}` | DENTISTRY SYMBOL LIGHT VERTICAL AND BOTTOM LEFT          |
| ⏍    | `\u{23CD}` | SQUARE FOOT                                              |
| ⏎    | `\u{23CE}` | RETURN SYMBOL                                            |
| ⏏    | `\u{23CF}` | EJECT SYMBOL                                             |
| ⏐    | `\u{23D0}` | VERTICAL LINE EXTENSION                                  |
| ⏑    | `\u{23D1}` | METRICAL BREVE                                           |
| ⏒    | `\u{23D2}` | METRICAL LONG OVER SHORT                                 |
| ⏓    | `\u{23D3}` | METRICAL SHORT OVER LONG                                 |
| ⏔    | `\u{23D4}` | METRICAL LONG OVER TWO SHORTS                            |
| ⏕    | `\u{23D5}` | METRICAL TWO SHORTS OVER LONG                            |
| ⏖    | `\u{23D6}` | METRICAL TWO SHORTS JOINED                               |
| ⏗    | `\u{23D7}` | METRICAL TRISEME                                         |
| ⏘    | `\u{23D8}` | METRICAL TETRASEME                                       |
| ⏙    | `\u{23D9}` | METRICAL PENTASEME                                       |
| ⏚    | `\u{23DA}` | EARTH GROUND                                             |
| ⏛    | `\u{23DB}` | FUSE                                                     |
| ⏜    | `\u{23DC}` | TOP PARENTHESIS                                          |
| ⏝    | `\u{23DD}` | BOTTOM PARENTHESIS                                       |
| ⏞    | `\u{23DE}` | TOP CURLY BRACKET                                        |
| ⏟    | `\u{23DF}` | BOTTOM CURLY BRACKET                                     |
| ⏠    | `\u{23E0}` | TOP TORTOISE SHELL BRACKET                               |
| ⏡    | `\u{23E1}` | BOTTOM TORTOISE SHELL BRACKET                            |
| ⏢    | `\u{23E2}` | WHITE TRAPEZIUM                                          |
| ⏣    | `\u{23E3}` | BENZENE RING WITH CIRCLE                                 |
| ⏤    | `\u{23E4}` | STRAIGHTNESS                                             |
| ⏥    | `\u{23E5}` | FLATNESS                                                 |
| ⏦    | `\u{23E6}` | AC CURRENT                                               |
| ⏧    | `\u{23E7}` | ELECTRICAL INTERSECTION                                  |
| ⏨    | `\u{23E8}` | DECIMAL EXPONENT SYMBOL                                  |
| ⏩   | `\u{23E9}` | BLACK RIGHT-POINTING DOUBLE TRIANGLE                     |
| ⏪   | `\u{23EA}` | BLACK LEFT-POINTING DOUBLE TRIANGLE                      |
| ⏫   | `\u{23EB}` | BLACK UP-POINTING DOUBLE TRIANGLE                        |
| ⏬   | `\u{23EC}` | BLACK DOWN-POINTING DOUBLE TRIANGLE                      |
| ⏭    | `\u{23ED}` | BLACK RIGHT-POINTING DOUBLE TRIANGLE WITH VERTICAL BAR   |
| ⏮    | `\u{23EE}` | BLACK LEFT-POINTING DOUBLE TRIANGLE WITH VERTICAL BAR    |
| ⏯    | `\u{23EF}` | BLACK RIGHT-POINTING TRIANGLE WITH DOUBLE VERTICAL BAR   |
| ⏰   | `\u{23F0}` | ALARM CLOCK                                              |
| ⏱    | `\u{23F1}` | STOPWATCH                                                |
| ⏲    | `\u{23F2}` | TIMER CLOCK                                              |
| ⏳   | `\u{23F3}` | HOURGLASS WITH FLOWING SAND                              |
| ⏴    | `\u{23F4}` | BLACK MEDIUM LEFT-POINTING TRIANGLE                      |
| ⏵    | `\u{23F5}` | BLACK MEDIUM RIGHT-POINTING TRIANGLE                     |
| ⏶    | `\u{23F6}` | BLACK MEDIUM UP-POINTING TRIANGLE                        |
| ⏷    | `\u{23F7}` | BLACK MEDIUM DOWN-POINTING TRIANGLE                      |
| ⏸    | `\u{23F8}` | DOUBLE VERTICAL BAR                                      |
| ⏹    | `\u{23F9}` | BLACK SQUARE FOR STOP                                    |
| ⏺    | `\u{23FA}` | BLACK CIRCLE FOR RECORD                                  |
| ⏻    | `\u{23FB}` | POWER SYMBOL                                             |
| ⏼    | `\u{23FC}` | POWER ON-OFF SYMBOL                                      |
| ⏽    | `\u{23FD}` | POWER ON SYMBOL                                          |
| ⏾    | `\u{23FE}` | POWER SLEEP SYMBOL                                       |
| ⏿    | `\u{23FF}` | OBSERVER EYE SYMBOL                                      |

## Dingbats (192)

| Char | Escape     | Name                                                |
|------|------------|-----------------------------------------------------|
| ✀    | `\u{2700}` | BLACK SAFETY SCISSORS                               |
| ✁    | `\u{2701}` | UPPER BLADE SCISSORS                                |
| ✂    | `\u{2702}` | BLACK SCISSORS                                      |
| ✃    | `\u{2703}` | LOWER BLADE SCISSORS                                |
| ✄    | `\u{2704}` | WHITE SCISSORS                                      |
| ✅   | `\u{2705}` | WHITE HEAVY CHECK MARK                              |
| ✆    | `\u{2706}` | TELEPHONE LOCATION SIGN                             |
| ✇    | `\u{2707}` | TAPE DRIVE                                          |
| ✈    | `\u{2708}` | AIRPLANE                                            |
| ✉    | `\u{2709}` | ENVELOPE                                            |
| ✊   | `\u{270A}` | RAISED FIST                                         |
| ✋   | `\u{270B}` | RAISED HAND                                         |
| ✌    | `\u{270C}` | VICTORY HAND                                        |
| ✍    | `\u{270D}` | WRITING HAND                                        |
| ✎    | `\u{270E}` | LOWER RIGHT PENCIL                                  |
| ✏    | `\u{270F}` | PENCIL                                              |
| ✐    | `\u{2710}` | UPPER RIGHT PENCIL                                  |
| ✑    | `\u{2711}` | WHITE NIB                                           |
| ✒    | `\u{2712}` | BLACK NIB                                           |
| ✓    | `\u{2713}` | CHECK MARK                                          |
| ✔    | `\u{2714}` | HEAVY CHECK MARK                                    |
| ✕    | `\u{2715}` | MULTIPLICATION X                                    |
| ✖    | `\u{2716}` | HEAVY MULTIPLICATION X                              |
| ✗    | `\u{2717}` | BALLOT X                                            |
| ✘    | `\u{2718}` | HEAVY BALLOT X                                      |
| ✙    | `\u{2719}` | OUTLINED GREEK CROSS                                |
| ✚    | `\u{271A}` | HEAVY GREEK CROSS                                   |
| ✛    | `\u{271B}` | OPEN CENTRE CROSS                                   |
| ✜    | `\u{271C}` | HEAVY OPEN CENTRE CROSS                             |
| ✝    | `\u{271D}` | LATIN CROSS                                         |
| ✞    | `\u{271E}` | SHADOWED WHITE LATIN CROSS                          |
| ✟    | `\u{271F}` | OUTLINED LATIN CROSS                                |
| ✠    | `\u{2720}` | MALTESE CROSS                                       |
| ✡    | `\u{2721}` | STAR OF DAVID                                       |
| ✢    | `\u{2722}` | FOUR TEARDROP-SPOKED ASTERISK                       |
| ✣    | `\u{2723}` | FOUR BALLOON-SPOKED ASTERISK                        |
| ✤    | `\u{2724}` | HEAVY FOUR BALLOON-SPOKED ASTERISK                  |
| ✥    | `\u{2725}` | FOUR CLUB-SPOKED ASTERISK                           |
| ✦    | `\u{2726}` | BLACK FOUR POINTED STAR                             |
| ✧    | `\u{2727}` | WHITE FOUR POINTED STAR                             |
| ✨   | `\u{2728}` | SPARKLES                                            |
| ✩    | `\u{2729}` | STRESS OUTLINED WHITE STAR                          |
| ✪    | `\u{272A}` | CIRCLED WHITE STAR                                  |
| ✫    | `\u{272B}` | OPEN CENTRE BLACK STAR                              |
| ✬    | `\u{272C}` | BLACK CENTRE WHITE STAR                             |
| ✭    | `\u{272D}` | OUTLINED BLACK STAR                                 |
| ✮    | `\u{272E}` | HEAVY OUTLINED BLACK STAR                           |
| ✯    | `\u{272F}` | PINWHEEL STAR                                       |
| ✰    | `\u{2730}` | SHADOWED WHITE STAR                                 |
| ✱    | `\u{2731}` | HEAVY ASTERISK                                      |
| ✲    | `\u{2732}` | OPEN CENTRE ASTERISK                                |
| ✳    | `\u{2733}` | EIGHT SPOKED ASTERISK                               |
| ✴    | `\u{2734}` | EIGHT POINTED BLACK STAR                            |
| ✵    | `\u{2735}` | EIGHT POINTED PINWHEEL STAR                         |
| ✶    | `\u{2736}` | SIX POINTED BLACK STAR                              |
| ✷    | `\u{2737}` | EIGHT POINTED RECTILINEAR BLACK STAR                |
| ✸    | `\u{2738}` | HEAVY EIGHT POINTED RECTILINEAR BLACK STAR          |
| ✹    | `\u{2739}` | TWELVE POINTED BLACK STAR                           |
| ✺    | `\u{273A}` | SIXTEEN POINTED ASTERISK                            |
| ✻    | `\u{273B}` | TEARDROP-SPOKED ASTERISK                            |
| ✼    | `\u{273C}` | OPEN CENTRE TEARDROP-SPOKED ASTERISK                |
| ✽    | `\u{273D}` | HEAVY TEARDROP-SPOKED ASTERISK                      |
| ✾    | `\u{273E}` | SIX PETALLED BLACK AND WHITE FLORETTE               |
| ✿    | `\u{273F}` | BLACK FLORETTE                                      |
| ❀    | `\u{2740}` | WHITE FLORETTE                                      |
| ❁    | `\u{2741}` | EIGHT PETALLED OUTLINED BLACK FLORETTE              |
| ❂    | `\u{2742}` | CIRCLED OPEN CENTRE EIGHT POINTED STAR              |
| ❃    | `\u{2743}` | HEAVY TEARDROP-SPOKED PINWHEEL ASTERISK             |
| ❄    | `\u{2744}` | SNOWFLAKE                                           |
| ❅    | `\u{2745}` | TIGHT TRIFOLIATE SNOWFLAKE                          |
| ❆    | `\u{2746}` | HEAVY CHEVRON SNOWFLAKE                             |
| ❇    | `\u{2747}` | SPARKLE                                             |
| ❈    | `\u{2748}` | HEAVY SPARKLE                                       |
| ❉    | `\u{2749}` | BALLOON-SPOKED ASTERISK                             |
| ❊    | `\u{274A}` | EIGHT TEARDROP-SPOKED PROPELLER ASTERISK            |
| ❋    | `\u{274B}` | HEAVY EIGHT TEARDROP-SPOKED PROPELLER ASTERISK      |
| ❌   | `\u{274C}` | CROSS MARK                                          |
| ❍    | `\u{274D}` | SHADOWED WHITE CIRCLE                               |
| ❎   | `\u{274E}` | NEGATIVE SQUARED CROSS MARK                         |
| ❏    | `\u{274F}` | LOWER RIGHT DROP-SHADOWED WHITE SQUARE              |
| ❐    | `\u{2750}` | UPPER RIGHT DROP-SHADOWED WHITE SQUARE              |
| ❑    | `\u{2751}` | LOWER RIGHT SHADOWED WHITE SQUARE                   |
| ❒    | `\u{2752}` | UPPER RIGHT SHADOWED WHITE SQUARE                   |
| ❓   | `\u{2753}` | BLACK QUESTION MARK ORNAMENT                        |
| ❔   | `\u{2754}` | WHITE QUESTION MARK ORNAMENT                        |
| ❕   | `\u{2755}` | WHITE EXCLAMATION MARK ORNAMENT                     |
| ❖    | `\u{2756}` | BLACK DIAMOND MINUS WHITE X                         |
| ❗   | `\u{2757}` | HEAVY EXCLAMATION MARK SYMBOL                       |
| ❘    | `\u{2758}` | LIGHT VERTICAL BAR                                  |
| ❙    | `\u{2759}` | MEDIUM VERTICAL BAR                                 |
| ❚    | `\u{275A}` | HEAVY VERTICAL BAR                                  |
| ❛    | `\u{275B}` | HEAVY SINGLE TURNED COMMA QUOTATION MARK ORNAMENT   |
| ❜    | `\u{275C}` | HEAVY SINGLE COMMA QUOTATION MARK ORNAMENT          |
| ❝    | `\u{275D}` | HEAVY DOUBLE TURNED COMMA QUOTATION MARK ORNAMENT   |
| ❞    | `\u{275E}` | HEAVY DOUBLE COMMA QUOTATION MARK ORNAMENT          |
| ❟    | `\u{275F}` | HEAVY LOW SINGLE COMMA QUOTATION MARK ORNAMENT      |
| ❠    | `\u{2760}` | HEAVY LOW DOUBLE COMMA QUOTATION MARK ORNAMENT      |
| ❡    | `\u{2761}` | CURVED STEM PARAGRAPH SIGN ORNAMENT                 |
| ❢    | `\u{2762}` | HEAVY EXCLAMATION MARK ORNAMENT                     |
| ❣    | `\u{2763}` | HEAVY HEART EXCLAMATION MARK ORNAMENT               |
| ❤    | `\u{2764}` | HEAVY BLACK HEART                                   |
| ❥    | `\u{2765}` | ROTATED HEAVY BLACK HEART BULLET                    |
| ❦    | `\u{2766}` | FLORAL HEART                                        |
| ❧    | `\u{2767}` | ROTATED FLORAL HEART BULLET                         |
| ❨    | `\u{2768}` | MEDIUM LEFT PARENTHESIS ORNAMENT                    |
| ❩    | `\u{2769}` | MEDIUM RIGHT PARENTHESIS ORNAMENT                   |
| ❪    | `\u{276A}` | MEDIUM FLATTENED LEFT PARENTHESIS ORNAMENT          |
| ❫    | `\u{276B}` | MEDIUM FLATTENED RIGHT PARENTHESIS ORNAMENT         |
| ❬    | `\u{276C}` | MEDIUM LEFT-POINTING ANGLE BRACKET ORNAMENT         |
| ❭    | `\u{276D}` | MEDIUM RIGHT-POINTING ANGLE BRACKET ORNAMENT        |
| ❮    | `\u{276E}` | HEAVY LEFT-POINTING ANGLE QUOTATION MARK ORNAMENT   |
| ❯    | `\u{276F}` | HEAVY RIGHT-POINTING ANGLE QUOTATION MARK ORNAMENT  |
| ❰    | `\u{2770}` | HEAVY LEFT-POINTING ANGLE BRACKET ORNAMENT          |
| ❱    | `\u{2771}` | HEAVY RIGHT-POINTING ANGLE BRACKET ORNAMENT         |
| ❲    | `\u{2772}` | LIGHT LEFT TORTOISE SHELL BRACKET ORNAMENT          |
| ❳    | `\u{2773}` | LIGHT RIGHT TORTOISE SHELL BRACKET ORNAMENT         |
| ❴    | `\u{2774}` | MEDIUM LEFT CURLY BRACKET ORNAMENT                  |
| ❵    | `\u{2775}` | MEDIUM RIGHT CURLY BRACKET ORNAMENT                 |
| ❶    | `\u{2776}` | DINGBAT NEGATIVE CIRCLED DIGIT ONE                  |
| ❷    | `\u{2777}` | DINGBAT NEGATIVE CIRCLED DIGIT TWO                  |
| ❸    | `\u{2778}` | DINGBAT NEGATIVE CIRCLED DIGIT THREE                |
| ❹    | `\u{2779}` | DINGBAT NEGATIVE CIRCLED DIGIT FOUR                 |
| ❺    | `\u{277A}` | DINGBAT NEGATIVE CIRCLED DIGIT FIVE                 |
| ❻    | `\u{277B}` | DINGBAT NEGATIVE CIRCLED DIGIT SIX                  |
| ❼    | `\u{277C}` | DINGBAT NEGATIVE CIRCLED DIGIT SEVEN                |
| ❽    | `\u{277D}` | DINGBAT NEGATIVE CIRCLED DIGIT EIGHT                |
| ❾    | `\u{277E}` | DINGBAT NEGATIVE CIRCLED DIGIT NINE                 |
| ❿    | `\u{277F}` | DINGBAT NEGATIVE CIRCLED NUMBER TEN                 |
| ➀    | `\u{2780}` | DINGBAT CIRCLED SANS-SERIF DIGIT ONE                |
| ➁    | `\u{2781}` | DINGBAT CIRCLED SANS-SERIF DIGIT TWO                |
| ➂    | `\u{2782}` | DINGBAT CIRCLED SANS-SERIF DIGIT THREE              |
| ➃    | `\u{2783}` | DINGBAT CIRCLED SANS-SERIF DIGIT FOUR               |
| ➄    | `\u{2784}` | DINGBAT CIRCLED SANS-SERIF DIGIT FIVE               |
| ➅    | `\u{2785}` | DINGBAT CIRCLED SANS-SERIF DIGIT SIX                |
| ➆    | `\u{2786}` | DINGBAT CIRCLED SANS-SERIF DIGIT SEVEN              |
| ➇    | `\u{2787}` | DINGBAT CIRCLED SANS-SERIF DIGIT EIGHT              |
| ➈    | `\u{2788}` | DINGBAT CIRCLED SANS-SERIF DIGIT NINE               |
| ➉    | `\u{2789}` | DINGBAT CIRCLED SANS-SERIF NUMBER TEN               |
| ➊    | `\u{278A}` | DINGBAT NEGATIVE CIRCLED SANS-SERIF DIGIT ONE       |
| ➋    | `\u{278B}` | DINGBAT NEGATIVE CIRCLED SANS-SERIF DIGIT TWO       |
| ➌    | `\u{278C}` | DINGBAT NEGATIVE CIRCLED SANS-SERIF DIGIT THREE     |
| ➍    | `\u{278D}` | DINGBAT NEGATIVE CIRCLED SANS-SERIF DIGIT FOUR      |
| ➎    | `\u{278E}` | DINGBAT NEGATIVE CIRCLED SANS-SERIF DIGIT FIVE      |
| ➏    | `\u{278F}` | DINGBAT NEGATIVE CIRCLED SANS-SERIF DIGIT SIX       |
| ➐    | `\u{2790}` | DINGBAT NEGATIVE CIRCLED SANS-SERIF DIGIT SEVEN     |
| ➑    | `\u{2791}` | DINGBAT NEGATIVE CIRCLED SANS-SERIF DIGIT EIGHT     |
| ➒    | `\u{2792}` | DINGBAT NEGATIVE CIRCLED SANS-SERIF DIGIT NINE      |
| ➓    | `\u{2793}` | DINGBAT NEGATIVE CIRCLED SANS-SERIF NUMBER TEN      |
| ➔    | `\u{2794}` | HEAVY WIDE-HEADED RIGHTWARDS ARROW                  |
| ➕   | `\u{2795}` | HEAVY PLUS SIGN                                     |
| ➖   | `\u{2796}` | HEAVY MINUS SIGN                                    |
| ➗   | `\u{2797}` | HEAVY DIVISION SIGN                                 |
| ➘    | `\u{2798}` | HEAVY SOUTH EAST ARROW                              |
| ➙    | `\u{2799}` | HEAVY RIGHTWARDS ARROW                              |
| ➚    | `\u{279A}` | HEAVY NORTH EAST ARROW                              |
| ➛    | `\u{279B}` | DRAFTING POINT RIGHTWARDS ARROW                     |
| ➜    | `\u{279C}` | HEAVY ROUND-TIPPED RIGHTWARDS ARROW                 |
| ➝    | `\u{279D}` | TRIANGLE-HEADED RIGHTWARDS ARROW                    |
| ➞    | `\u{279E}` | HEAVY TRIANGLE-HEADED RIGHTWARDS ARROW              |
| ➟    | `\u{279F}` | DASHED TRIANGLE-HEADED RIGHTWARDS ARROW             |
| ➠    | `\u{27A0}` | HEAVY DASHED TRIANGLE-HEADED RIGHTWARDS ARROW       |
| ➡    | `\u{27A1}` | BLACK RIGHTWARDS ARROW                              |
| ➢    | `\u{27A2}` | THREE-D TOP-LIGHTED RIGHTWARDS ARROWHEAD            |
| ➣    | `\u{27A3}` | THREE-D BOTTOM-LIGHTED RIGHTWARDS ARROWHEAD         |
| ➤    | `\u{27A4}` | BLACK RIGHTWARDS ARROWHEAD                          |
| ➥    | `\u{27A5}` | HEAVY BLACK CURVED DOWNWARDS AND RIGHTWARDS ARROW   |
| ➦    | `\u{27A6}` | HEAVY BLACK CURVED UPWARDS AND RIGHTWARDS ARROW     |
| ➧    | `\u{27A7}` | SQUAT BLACK RIGHTWARDS ARROW                        |
| ➨    | `\u{27A8}` | HEAVY CONCAVE-POINTED BLACK RIGHTWARDS ARROW        |
| ➩    | `\u{27A9}` | RIGHT-SHADED WHITE RIGHTWARDS ARROW                 |
| ➪    | `\u{27AA}` | LEFT-SHADED WHITE RIGHTWARDS ARROW                  |
| ➫    | `\u{27AB}` | BACK-TILTED SHADOWED WHITE RIGHTWARDS ARROW         |
| ➬    | `\u{27AC}` | FRONT-TILTED SHADOWED WHITE RIGHTWARDS ARROW        |
| ➭    | `\u{27AD}` | HEAVY LOWER RIGHT-SHADOWED WHITE RIGHTWARDS ARROW   |
| ➮    | `\u{27AE}` | HEAVY UPPER RIGHT-SHADOWED WHITE RIGHTWARDS ARROW   |
| ➯    | `\u{27AF}` | NOTCHED LOWER RIGHT-SHADOWED WHITE RIGHTWARDS ARROW |
| ➰   | `\u{27B0}` | CURLY LOOP                                          |
| ➱    | `\u{27B1}` | NOTCHED UPPER RIGHT-SHADOWED WHITE RIGHTWARDS ARROW |
| ➲    | `\u{27B2}` | CIRCLED HEAVY WHITE RIGHTWARDS ARROW                |
| ➳    | `\u{27B3}` | WHITE-FEATHERED RIGHTWARDS ARROW                    |
| ➴    | `\u{27B4}` | BLACK-FEATHERED SOUTH EAST ARROW                    |
| ➵    | `\u{27B5}` | BLACK-FEATHERED RIGHTWARDS ARROW                    |
| ➶    | `\u{27B6}` | BLACK-FEATHERED NORTH EAST ARROW                    |
| ➷    | `\u{27B7}` | HEAVY BLACK-FEATHERED SOUTH EAST ARROW              |
| ➸    | `\u{27B8}` | HEAVY BLACK-FEATHERED RIGHTWARDS ARROW              |
| ➹    | `\u{27B9}` | HEAVY BLACK-FEATHERED NORTH EAST ARROW              |
| ➺    | `\u{27BA}` | TEARDROP-BARBED RIGHTWARDS ARROW                    |
| ➻    | `\u{27BB}` | HEAVY TEARDROP-SHANKED RIGHTWARDS ARROW             |
| ➼    | `\u{27BC}` | WEDGE-TAILED RIGHTWARDS ARROW                       |
| ➽    | `\u{27BD}` | HEAVY WEDGE-TAILED RIGHTWARDS ARROW                 |
| ➾    | `\u{27BE}` | OPEN-OUTLINED RIGHTWARDS ARROW                      |
| ➿   | `\u{27BF}` | DOUBLE CURLY LOOP                                   |

## Enclosed Alphanumerics (160)

| Char | Escape     | Name                               |
|------|------------|------------------------------------|
| ①    | `\u{2460}` | CIRCLED DIGIT ONE                  |
| ②    | `\u{2461}` | CIRCLED DIGIT TWO                  |
| ③    | `\u{2462}` | CIRCLED DIGIT THREE                |
| ④    | `\u{2463}` | CIRCLED DIGIT FOUR                 |
| ⑤    | `\u{2464}` | CIRCLED DIGIT FIVE                 |
| ⑥    | `\u{2465}` | CIRCLED DIGIT SIX                  |
| ⑦    | `\u{2466}` | CIRCLED DIGIT SEVEN                |
| ⑧    | `\u{2467}` | CIRCLED DIGIT EIGHT                |
| ⑨    | `\u{2468}` | CIRCLED DIGIT NINE                 |
| ⑩    | `\u{2469}` | CIRCLED NUMBER TEN                 |
| ⑪    | `\u{246A}` | CIRCLED NUMBER ELEVEN              |
| ⑫    | `\u{246B}` | CIRCLED NUMBER TWELVE              |
| ⑬    | `\u{246C}` | CIRCLED NUMBER THIRTEEN            |
| ⑭    | `\u{246D}` | CIRCLED NUMBER FOURTEEN            |
| ⑮    | `\u{246E}` | CIRCLED NUMBER FIFTEEN             |
| ⑯    | `\u{246F}` | CIRCLED NUMBER SIXTEEN             |
| ⑰    | `\u{2470}` | CIRCLED NUMBER SEVENTEEN           |
| ⑱    | `\u{2471}` | CIRCLED NUMBER EIGHTEEN            |
| ⑲    | `\u{2472}` | CIRCLED NUMBER NINETEEN            |
| ⑳    | `\u{2473}` | CIRCLED NUMBER TWENTY              |
| ⑴    | `\u{2474}` | PARENTHESIZED DIGIT ONE            |
| ⑵    | `\u{2475}` | PARENTHESIZED DIGIT TWO            |
| ⑶    | `\u{2476}` | PARENTHESIZED DIGIT THREE          |
| ⑷    | `\u{2477}` | PARENTHESIZED DIGIT FOUR           |
| ⑸    | `\u{2478}` | PARENTHESIZED DIGIT FIVE           |
| ⑹    | `\u{2479}` | PARENTHESIZED DIGIT SIX            |
| ⑺    | `\u{247A}` | PARENTHESIZED DIGIT SEVEN          |
| ⑻    | `\u{247B}` | PARENTHESIZED DIGIT EIGHT          |
| ⑼    | `\u{247C}` | PARENTHESIZED DIGIT NINE           |
| ⑽    | `\u{247D}` | PARENTHESIZED NUMBER TEN           |
| ⑾    | `\u{247E}` | PARENTHESIZED NUMBER ELEVEN        |
| ⑿    | `\u{247F}` | PARENTHESIZED NUMBER TWELVE        |
| ⒀    | `\u{2480}` | PARENTHESIZED NUMBER THIRTEEN      |
| ⒁    | `\u{2481}` | PARENTHESIZED NUMBER FOURTEEN      |
| ⒂    | `\u{2482}` | PARENTHESIZED NUMBER FIFTEEN       |
| ⒃    | `\u{2483}` | PARENTHESIZED NUMBER SIXTEEN       |
| ⒄    | `\u{2484}` | PARENTHESIZED NUMBER SEVENTEEN     |
| ⒅    | `\u{2485}` | PARENTHESIZED NUMBER EIGHTEEN      |
| ⒆    | `\u{2486}` | PARENTHESIZED NUMBER NINETEEN      |
| ⒇    | `\u{2487}` | PARENTHESIZED NUMBER TWENTY        |
| ⒈    | `\u{2488}` | DIGIT ONE FULL STOP                |
| ⒉    | `\u{2489}` | DIGIT TWO FULL STOP                |
| ⒊    | `\u{248A}` | DIGIT THREE FULL STOP              |
| ⒋    | `\u{248B}` | DIGIT FOUR FULL STOP               |
| ⒌    | `\u{248C}` | DIGIT FIVE FULL STOP               |
| ⒍    | `\u{248D}` | DIGIT SIX FULL STOP                |
| ⒎    | `\u{248E}` | DIGIT SEVEN FULL STOP              |
| ⒏    | `\u{248F}` | DIGIT EIGHT FULL STOP              |
| ⒐    | `\u{2490}` | DIGIT NINE FULL STOP               |
| ⒑    | `\u{2491}` | NUMBER TEN FULL STOP               |
| ⒒    | `\u{2492}` | NUMBER ELEVEN FULL STOP            |
| ⒓    | `\u{2493}` | NUMBER TWELVE FULL STOP            |
| ⒔    | `\u{2494}` | NUMBER THIRTEEN FULL STOP          |
| ⒕    | `\u{2495}` | NUMBER FOURTEEN FULL STOP          |
| ⒖    | `\u{2496}` | NUMBER FIFTEEN FULL STOP           |
| ⒗    | `\u{2497}` | NUMBER SIXTEEN FULL STOP           |
| ⒘    | `\u{2498}` | NUMBER SEVENTEEN FULL STOP         |
| ⒙    | `\u{2499}` | NUMBER EIGHTEEN FULL STOP          |
| ⒚    | `\u{249A}` | NUMBER NINETEEN FULL STOP          |
| ⒛    | `\u{249B}` | NUMBER TWENTY FULL STOP            |
| ⒜    | `\u{249C}` | PARENTHESIZED LATIN SMALL LETTER A |
| ⒝    | `\u{249D}` | PARENTHESIZED LATIN SMALL LETTER B |
| ⒞    | `\u{249E}` | PARENTHESIZED LATIN SMALL LETTER C |
| ⒟    | `\u{249F}` | PARENTHESIZED LATIN SMALL LETTER D |
| ⒠    | `\u{24A0}` | PARENTHESIZED LATIN SMALL LETTER E |
| ⒡    | `\u{24A1}` | PARENTHESIZED LATIN SMALL LETTER F |
| ⒢    | `\u{24A2}` | PARENTHESIZED LATIN SMALL LETTER G |
| ⒣    | `\u{24A3}` | PARENTHESIZED LATIN SMALL LETTER H |
| ⒤    | `\u{24A4}` | PARENTHESIZED LATIN SMALL LETTER I |
| ⒥    | `\u{24A5}` | PARENTHESIZED LATIN SMALL LETTER J |
| ⒦    | `\u{24A6}` | PARENTHESIZED LATIN SMALL LETTER K |
| ⒧    | `\u{24A7}` | PARENTHESIZED LATIN SMALL LETTER L |
| ⒨    | `\u{24A8}` | PARENTHESIZED LATIN SMALL LETTER M |
| ⒩    | `\u{24A9}` | PARENTHESIZED LATIN SMALL LETTER N |
| ⒪    | `\u{24AA}` | PARENTHESIZED LATIN SMALL LETTER O |
| ⒫    | `\u{24AB}` | PARENTHESIZED LATIN SMALL LETTER P |
| ⒬    | `\u{24AC}` | PARENTHESIZED LATIN SMALL LETTER Q |
| ⒭    | `\u{24AD}` | PARENTHESIZED LATIN SMALL LETTER R |
| ⒮    | `\u{24AE}` | PARENTHESIZED LATIN SMALL LETTER S |
| ⒯    | `\u{24AF}` | PARENTHESIZED LATIN SMALL LETTER T |
| ⒰    | `\u{24B0}` | PARENTHESIZED LATIN SMALL LETTER U |
| ⒱    | `\u{24B1}` | PARENTHESIZED LATIN SMALL LETTER V |
| ⒲    | `\u{24B2}` | PARENTHESIZED LATIN SMALL LETTER W |
| ⒳    | `\u{24B3}` | PARENTHESIZED LATIN SMALL LETTER X |
| ⒴    | `\u{24B4}` | PARENTHESIZED LATIN SMALL LETTER Y |
| ⒵    | `\u{24B5}` | PARENTHESIZED LATIN SMALL LETTER Z |
| Ⓐ    | `\u{24B6}` | CIRCLED LATIN CAPITAL LETTER A     |
| Ⓑ    | `\u{24B7}` | CIRCLED LATIN CAPITAL LETTER B     |
| Ⓒ    | `\u{24B8}` | CIRCLED LATIN CAPITAL LETTER C     |
| Ⓓ    | `\u{24B9}` | CIRCLED LATIN CAPITAL LETTER D     |
| Ⓔ    | `\u{24BA}` | CIRCLED LATIN CAPITAL LETTER E     |
| Ⓕ    | `\u{24BB}` | CIRCLED LATIN CAPITAL LETTER F     |
| Ⓖ    | `\u{24BC}` | CIRCLED LATIN CAPITAL LETTER G     |
| Ⓗ    | `\u{24BD}` | CIRCLED LATIN CAPITAL LETTER H     |
| Ⓘ    | `\u{24BE}` | CIRCLED LATIN CAPITAL LETTER I     |
| Ⓙ    | `\u{24BF}` | CIRCLED LATIN CAPITAL LETTER J     |
| Ⓚ    | `\u{24C0}` | CIRCLED LATIN CAPITAL LETTER K     |
| Ⓛ    | `\u{24C1}` | CIRCLED LATIN CAPITAL LETTER L     |
| Ⓜ    | `\u{24C2}` | CIRCLED LATIN CAPITAL LETTER M     |
| Ⓝ    | `\u{24C3}` | CIRCLED LATIN CAPITAL LETTER N     |
| Ⓞ    | `\u{24C4}` | CIRCLED LATIN CAPITAL LETTER O     |
| Ⓟ    | `\u{24C5}` | CIRCLED LATIN CAPITAL LETTER P     |
| Ⓠ    | `\u{24C6}` | CIRCLED LATIN CAPITAL LETTER Q     |
| Ⓡ    | `\u{24C7}` | CIRCLED LATIN CAPITAL LETTER R     |
| Ⓢ    | `\u{24C8}` | CIRCLED LATIN CAPITAL LETTER S     |
| Ⓣ    | `\u{24C9}` | CIRCLED LATIN CAPITAL LETTER T     |
| Ⓤ    | `\u{24CA}` | CIRCLED LATIN CAPITAL LETTER U     |
| Ⓥ    | `\u{24CB}` | CIRCLED LATIN CAPITAL LETTER V     |
| Ⓦ    | `\u{24CC}` | CIRCLED LATIN CAPITAL LETTER W     |
| Ⓧ    | `\u{24CD}` | CIRCLED LATIN CAPITAL LETTER X     |
| Ⓨ    | `\u{24CE}` | CIRCLED LATIN CAPITAL LETTER Y     |
| Ⓩ    | `\u{24CF}` | CIRCLED LATIN CAPITAL LETTER Z     |
| ⓐ    | `\u{24D0}` | CIRCLED LATIN SMALL LETTER A       |
| ⓑ    | `\u{24D1}` | CIRCLED LATIN SMALL LETTER B       |
| ⓒ    | `\u{24D2}` | CIRCLED LATIN SMALL LETTER C       |
| ⓓ    | `\u{24D3}` | CIRCLED LATIN SMALL LETTER D       |
| ⓔ    | `\u{24D4}` | CIRCLED LATIN SMALL LETTER E       |
| ⓕ    | `\u{24D5}` | CIRCLED LATIN SMALL LETTER F       |
| ⓖ    | `\u{24D6}` | CIRCLED LATIN SMALL LETTER G       |
| ⓗ    | `\u{24D7}` | CIRCLED LATIN SMALL LETTER H       |
| ⓘ    | `\u{24D8}` | CIRCLED LATIN SMALL LETTER I       |
| ⓙ    | `\u{24D9}` | CIRCLED LATIN SMALL LETTER J       |
| ⓚ    | `\u{24DA}` | CIRCLED LATIN SMALL LETTER K       |
| ⓛ    | `\u{24DB}` | CIRCLED LATIN SMALL LETTER L       |
| ⓜ    | `\u{24DC}` | CIRCLED LATIN SMALL LETTER M       |
| ⓝ    | `\u{24DD}` | CIRCLED LATIN SMALL LETTER N       |
| ⓞ    | `\u{24DE}` | CIRCLED LATIN SMALL LETTER O       |
| ⓟ    | `\u{24DF}` | CIRCLED LATIN SMALL LETTER P       |
| ⓠ    | `\u{24E0}` | CIRCLED LATIN SMALL LETTER Q       |
| ⓡ    | `\u{24E1}` | CIRCLED LATIN SMALL LETTER R       |
| ⓢ    | `\u{24E2}` | CIRCLED LATIN SMALL LETTER S       |
| ⓣ    | `\u{24E3}` | CIRCLED LATIN SMALL LETTER T       |
| ⓤ    | `\u{24E4}` | CIRCLED LATIN SMALL LETTER U       |
| ⓥ    | `\u{24E5}` | CIRCLED LATIN SMALL LETTER V       |
| ⓦ    | `\u{24E6}` | CIRCLED LATIN SMALL LETTER W       |
| ⓧ    | `\u{24E7}` | CIRCLED LATIN SMALL LETTER X       |
| ⓨ    | `\u{24E8}` | CIRCLED LATIN SMALL LETTER Y       |
| ⓩ    | `\u{24E9}` | CIRCLED LATIN SMALL LETTER Z       |
| ⓪    | `\u{24EA}` | CIRCLED DIGIT ZERO                 |
| ⓫    | `\u{24EB}` | NEGATIVE CIRCLED NUMBER ELEVEN     |
| ⓬    | `\u{24EC}` | NEGATIVE CIRCLED NUMBER TWELVE     |
| ⓭    | `\u{24ED}` | NEGATIVE CIRCLED NUMBER THIRTEEN   |
| ⓮    | `\u{24EE}` | NEGATIVE CIRCLED NUMBER FOURTEEN   |
| ⓯    | `\u{24EF}` | NEGATIVE CIRCLED NUMBER FIFTEEN    |
| ⓰    | `\u{24F0}` | NEGATIVE CIRCLED NUMBER SIXTEEN    |
| ⓱    | `\u{24F1}` | NEGATIVE CIRCLED NUMBER SEVENTEEN  |
| ⓲    | `\u{24F2}` | NEGATIVE CIRCLED NUMBER EIGHTEEN   |
| ⓳    | `\u{24F3}` | NEGATIVE CIRCLED NUMBER NINETEEN   |
| ⓴    | `\u{24F4}` | NEGATIVE CIRCLED NUMBER TWENTY     |
| ⓵    | `\u{24F5}` | DOUBLE CIRCLED DIGIT ONE           |
| ⓶    | `\u{24F6}` | DOUBLE CIRCLED DIGIT TWO           |
| ⓷    | `\u{24F7}` | DOUBLE CIRCLED DIGIT THREE         |
| ⓸    | `\u{24F8}` | DOUBLE CIRCLED DIGIT FOUR          |
| ⓹    | `\u{24F9}` | DOUBLE CIRCLED DIGIT FIVE          |
| ⓺    | `\u{24FA}` | DOUBLE CIRCLED DIGIT SIX           |
| ⓻    | `\u{24FB}` | DOUBLE CIRCLED DIGIT SEVEN         |
| ⓼    | `\u{24FC}` | DOUBLE CIRCLED DIGIT EIGHT         |
| ⓽    | `\u{24FD}` | DOUBLE CIRCLED DIGIT NINE          |
| ⓾    | `\u{24FE}` | DOUBLE CIRCLED NUMBER TEN          |
| ⓿    | `\u{24FF}` | NEGATIVE CIRCLED DIGIT ZERO        |

## Miscellaneous Symbols (256)

| Char | Escape     | Name                                             |
|------|------------|--------------------------------------------------|
| ☀    | `\u{2600}` | BLACK SUN WITH RAYS                              |
| ☁    | `\u{2601}` | CLOUD                                            |
| ☂    | `\u{2602}` | UMBRELLA                                         |
| ☃    | `\u{2603}` | SNOWMAN                                          |
| ☄    | `\u{2604}` | COMET                                            |
| ★    | `\u{2605}` | BLACK STAR                                       |
| ☆    | `\u{2606}` | WHITE STAR                                       |
| ☇    | `\u{2607}` | LIGHTNING                                        |
| ☈    | `\u{2608}` | THUNDERSTORM                                     |
| ☉    | `\u{2609}` | SUN                                              |
| ☊    | `\u{260A}` | ASCENDING NODE                                   |
| ☋    | `\u{260B}` | DESCENDING NODE                                  |
| ☌    | `\u{260C}` | CONJUNCTION                                      |
| ☍    | `\u{260D}` | OPPOSITION                                       |
| ☎    | `\u{260E}` | BLACK TELEPHONE                                  |
| ☏    | `\u{260F}` | WHITE TELEPHONE                                  |
| ☐    | `\u{2610}` | BALLOT BOX                                       |
| ☑    | `\u{2611}` | BALLOT BOX WITH CHECK                            |
| ☒    | `\u{2612}` | BALLOT BOX WITH X                                |
| ☓    | `\u{2613}` | SALTIRE                                          |
| ☔   | `\u{2614}` | UMBRELLA WITH RAIN DROPS                         |
| ☕   | `\u{2615}` | HOT BEVERAGE                                     |
| ☖    | `\u{2616}` | WHITE SHOGI PIECE                                |
| ☗    | `\u{2617}` | BLACK SHOGI PIECE                                |
| ☘    | `\u{2618}` | SHAMROCK                                         |
| ☙    | `\u{2619}` | REVERSED ROTATED FLORAL HEART BULLET             |
| ☚    | `\u{261A}` | BLACK LEFT POINTING INDEX                        |
| ☛    | `\u{261B}` | BLACK RIGHT POINTING INDEX                       |
| ☜    | `\u{261C}` | WHITE LEFT POINTING INDEX                        |
| ☝    | `\u{261D}` | WHITE UP POINTING INDEX                          |
| ☞    | `\u{261E}` | WHITE RIGHT POINTING INDEX                       |
| ☟    | `\u{261F}` | WHITE DOWN POINTING INDEX                        |
| ☠    | `\u{2620}` | SKULL AND CROSSBONES                             |
| ☡    | `\u{2621}` | CAUTION SIGN                                     |
| ☢    | `\u{2622}` | RADIOACTIVE SIGN                                 |
| ☣    | `\u{2623}` | BIOHAZARD SIGN                                   |
| ☤    | `\u{2624}` | CADUCEUS                                         |
| ☥    | `\u{2625}` | ANKH                                             |
| ☦    | `\u{2626}` | ORTHODOX CROSS                                   |
| ☧    | `\u{2627}` | CHI RHO                                          |
| ☨    | `\u{2628}` | CROSS OF LORRAINE                                |
| ☩    | `\u{2629}` | CROSS OF JERUSALEM                               |
| ☪    | `\u{262A}` | STAR AND CRESCENT                                |
| ☫    | `\u{262B}` | FARSI SYMBOL                                     |
| ☬    | `\u{262C}` | ADI SHAKTI                                       |
| ☭    | `\u{262D}` | HAMMER AND SICKLE                                |
| ☮    | `\u{262E}` | PEACE SYMBOL                                     |
| ☯    | `\u{262F}` | YIN YANG                                         |
| ☰   | `\u{2630}` | TRIGRAM FOR HEAVEN                               |
| ☱   | `\u{2631}` | TRIGRAM FOR LAKE                                 |
| ☲   | `\u{2632}` | TRIGRAM FOR FIRE                                 |
| ☳   | `\u{2633}` | TRIGRAM FOR THUNDER                              |
| ☴   | `\u{2634}` | TRIGRAM FOR WIND                                 |
| ☵   | `\u{2635}` | TRIGRAM FOR WATER                                |
| ☶   | `\u{2636}` | TRIGRAM FOR MOUNTAIN                             |
| ☷   | `\u{2637}` | TRIGRAM FOR EARTH                                |
| ☸    | `\u{2638}` | WHEEL OF DHARMA                                  |
| ☹    | `\u{2639}` | WHITE FROWNING FACE                              |
| ☺    | `\u{263A}` | WHITE SMILING FACE                               |
| ☻    | `\u{263B}` | BLACK SMILING FACE                               |
| ☼    | `\u{263C}` | WHITE SUN WITH RAYS                              |
| ☽    | `\u{263D}` | FIRST QUARTER MOON                               |
| ☾    | `\u{263E}` | LAST QUARTER MOON                                |
| ☿    | `\u{263F}` | MERCURY                                          |
| ♀    | `\u{2640}` | FEMALE SIGN                                      |
| ♁    | `\u{2641}` | EARTH                                            |
| ♂    | `\u{2642}` | MALE SIGN                                        |
| ♃    | `\u{2643}` | JUPITER                                          |
| ♄    | `\u{2644}` | SATURN                                           |
| ♅    | `\u{2645}` | URANUS                                           |
| ♆    | `\u{2646}` | NEPTUNE                                          |
| ♇    | `\u{2647}` | PLUTO                                            |
| ♈   | `\u{2648}` | ARIES                                            |
| ♉   | `\u{2649}` | TAURUS                                           |
| ♊   | `\u{264A}` | GEMINI                                           |
| ♋   | `\u{264B}` | CANCER                                           |
| ♌   | `\u{264C}` | LEO                                              |
| ♍   | `\u{264D}` | VIRGO                                            |
| ♎   | `\u{264E}` | LIBRA                                            |
| ♏   | `\u{264F}` | SCORPIUS                                         |
| ♐   | `\u{2650}` | SAGITTARIUS                                      |
| ♑   | `\u{2651}` | CAPRICORN                                        |
| ♒   | `\u{2652}` | AQUARIUS                                         |
| ♓   | `\u{2653}` | PISCES                                           |
| ♔    | `\u{2654}` | WHITE CHESS KING                                 |
| ♕    | `\u{2655}` | WHITE CHESS QUEEN                                |
| ♖    | `\u{2656}` | WHITE CHESS ROOK                                 |
| ♗    | `\u{2657}` | WHITE CHESS BISHOP                               |
| ♘    | `\u{2658}` | WHITE CHESS KNIGHT                               |
| ♙    | `\u{2659}` | WHITE CHESS PAWN                                 |
| ♚    | `\u{265A}` | BLACK CHESS KING                                 |
| ♛    | `\u{265B}` | BLACK CHESS QUEEN                                |
| ♜    | `\u{265C}` | BLACK CHESS ROOK                                 |
| ♝    | `\u{265D}` | BLACK CHESS BISHOP                               |
| ♞    | `\u{265E}` | BLACK CHESS KNIGHT                               |
| ♟    | `\u{265F}` | BLACK CHESS PAWN                                 |
| ♠    | `\u{2660}` | BLACK SPADE SUIT                                 |
| ♡    | `\u{2661}` | WHITE HEART SUIT                                 |
| ♢    | `\u{2662}` | WHITE DIAMOND SUIT                               |
| ♣    | `\u{2663}` | BLACK CLUB SUIT                                  |
| ♤    | `\u{2664}` | WHITE SPADE SUIT                                 |
| ♥    | `\u{2665}` | BLACK HEART SUIT                                 |
| ♦    | `\u{2666}` | BLACK DIAMOND SUIT                               |
| ♧    | `\u{2667}` | WHITE CLUB SUIT                                  |
| ♨    | `\u{2668}` | HOT SPRINGS                                      |
| ♩    | `\u{2669}` | QUARTER NOTE                                     |
| ♪    | `\u{266A}` | EIGHTH NOTE                                      |
| ♫    | `\u{266B}` | BEAMED EIGHTH NOTES                              |
| ♬    | `\u{266C}` | BEAMED SIXTEENTH NOTES                           |
| ♭    | `\u{266D}` | MUSIC FLAT SIGN                                  |
| ♮    | `\u{266E}` | MUSIC NATURAL SIGN                               |
| ♯    | `\u{266F}` | MUSIC SHARP SIGN                                 |
| ♰    | `\u{2670}` | WEST SYRIAC CROSS                                |
| ♱    | `\u{2671}` | EAST SYRIAC CROSS                                |
| ♲    | `\u{2672}` | UNIVERSAL RECYCLING SYMBOL                       |
| ♳    | `\u{2673}` | RECYCLING SYMBOL FOR TYPE-1 PLASTICS             |
| ♴    | `\u{2674}` | RECYCLING SYMBOL FOR TYPE-2 PLASTICS             |
| ♵    | `\u{2675}` | RECYCLING SYMBOL FOR TYPE-3 PLASTICS             |
| ♶    | `\u{2676}` | RECYCLING SYMBOL FOR TYPE-4 PLASTICS             |
| ♷    | `\u{2677}` | RECYCLING SYMBOL FOR TYPE-5 PLASTICS             |
| ♸    | `\u{2678}` | RECYCLING SYMBOL FOR TYPE-6 PLASTICS             |
| ♹    | `\u{2679}` | RECYCLING SYMBOL FOR TYPE-7 PLASTICS             |
| ♺    | `\u{267A}` | RECYCLING SYMBOL FOR GENERIC MATERIALS           |
| ♻    | `\u{267B}` | BLACK UNIVERSAL RECYCLING SYMBOL                 |
| ♼    | `\u{267C}` | RECYCLED PAPER SYMBOL                            |
| ♽    | `\u{267D}` | PARTIALLY-RECYCLED PAPER SYMBOL                  |
| ♾    | `\u{267E}` | PERMANENT PAPER SIGN                             |
| ♿   | `\u{267F}` | WHEELCHAIR SYMBOL                                |
| ⚀    | `\u{2680}` | DIE FACE-1                                       |
| ⚁    | `\u{2681}` | DIE FACE-2                                       |
| ⚂    | `\u{2682}` | DIE FACE-3                                       |
| ⚃    | `\u{2683}` | DIE FACE-4                                       |
| ⚄    | `\u{2684}` | DIE FACE-5                                       |
| ⚅    | `\u{2685}` | DIE FACE-6                                       |
| ⚆    | `\u{2686}` | WHITE CIRCLE WITH DOT RIGHT                      |
| ⚇    | `\u{2687}` | WHITE CIRCLE WITH TWO DOTS                       |
| ⚈    | `\u{2688}` | BLACK CIRCLE WITH WHITE DOT RIGHT                |
| ⚉    | `\u{2689}` | BLACK CIRCLE WITH TWO WHITE DOTS                 |
| ⚊   | `\u{268A}` | MONOGRAM FOR YANG                                |
| ⚋   | `\u{268B}` | MONOGRAM FOR YIN                                 |
| ⚌   | `\u{268C}` | DIGRAM FOR GREATER YANG                          |
| ⚍   | `\u{268D}` | DIGRAM FOR LESSER YIN                            |
| ⚎   | `\u{268E}` | DIGRAM FOR LESSER YANG                           |
| ⚏   | `\u{268F}` | DIGRAM FOR GREATER YIN                           |
| ⚐    | `\u{2690}` | WHITE FLAG                                       |
| ⚑    | `\u{2691}` | BLACK FLAG                                       |
| ⚒    | `\u{2692}` | HAMMER AND PICK                                  |
| ⚓   | `\u{2693}` | ANCHOR                                           |
| ⚔    | `\u{2694}` | CROSSED SWORDS                                   |
| ⚕    | `\u{2695}` | STAFF OF AESCULAPIUS                             |
| ⚖    | `\u{2696}` | SCALES                                           |
| ⚗    | `\u{2697}` | ALEMBIC                                          |
| ⚘    | `\u{2698}` | FLOWER                                           |
| ⚙    | `\u{2699}` | GEAR                                             |
| ⚚    | `\u{269A}` | STAFF OF HERMES                                  |
| ⚛    | `\u{269B}` | ATOM SYMBOL                                      |
| ⚜    | `\u{269C}` | FLEUR-DE-LIS                                     |
| ⚝    | `\u{269D}` | OUTLINED WHITE STAR                              |
| ⚞    | `\u{269E}` | THREE LINES CONVERGING RIGHT                     |
| ⚟    | `\u{269F}` | THREE LINES CONVERGING LEFT                      |
| ⚠    | `\u{26A0}` | WARNING SIGN                                     |
| ⚡   | `\u{26A1}` | HIGH VOLTAGE SIGN                                |
| ⚢    | `\u{26A2}` | DOUBLED FEMALE SIGN                              |
| ⚣    | `\u{26A3}` | DOUBLED MALE SIGN                                |
| ⚤    | `\u{26A4}` | INTERLOCKED FEMALE AND MALE SIGN                 |
| ⚥    | `\u{26A5}` | MALE AND FEMALE SIGN                             |
| ⚦    | `\u{26A6}` | MALE WITH STROKE SIGN                            |
| ⚧    | `\u{26A7}` | MALE WITH STROKE AND MALE AND FEMALE SIGN        |
| ⚨    | `\u{26A8}` | VERTICAL MALE WITH STROKE SIGN                   |
| ⚩    | `\u{26A9}` | HORIZONTAL MALE WITH STROKE SIGN                 |
| ⚪   | `\u{26AA}` | MEDIUM WHITE CIRCLE                              |
| ⚫   | `\u{26AB}` | MEDIUM BLACK CIRCLE                              |
| ⚬    | `\u{26AC}` | MEDIUM SMALL WHITE CIRCLE                        |
| ⚭    | `\u{26AD}` | MARRIAGE SYMBOL                                  |
| ⚮    | `\u{26AE}` | DIVORCE SYMBOL                                   |
| ⚯    | `\u{26AF}` | UNMARRIED PARTNERSHIP SYMBOL                     |
| ⚰    | `\u{26B0}` | COFFIN                                           |
| ⚱    | `\u{26B1}` | FUNERAL URN                                      |
| ⚲    | `\u{26B2}` | NEUTER                                           |
| ⚳    | `\u{26B3}` | CERES                                            |
| ⚴    | `\u{26B4}` | PALLAS                                           |
| ⚵    | `\u{26B5}` | JUNO                                             |
| ⚶    | `\u{26B6}` | VESTA                                            |
| ⚷    | `\u{26B7}` | CHIRON                                           |
| ⚸    | `\u{26B8}` | BLACK MOON LILITH                                |
| ⚹    | `\u{26B9}` | SEXTILE                                          |
| ⚺    | `\u{26BA}` | SEMISEXTILE                                      |
| ⚻    | `\u{26BB}` | QUINCUNX                                         |
| ⚼    | `\u{26BC}` | SESQUIQUADRATE                                   |
| ⚽   | `\u{26BD}` | SOCCER BALL                                      |
| ⚾   | `\u{26BE}` | BASEBALL                                         |
| ⚿    | `\u{26BF}` | SQUARED KEY                                      |
| ⛀    | `\u{26C0}` | WHITE DRAUGHTS MAN                               |
| ⛁    | `\u{26C1}` | WHITE DRAUGHTS KING                              |
| ⛂    | `\u{26C2}` | BLACK DRAUGHTS MAN                               |
| ⛃    | `\u{26C3}` | BLACK DRAUGHTS KING                              |
| ⛄   | `\u{26C4}` | SNOWMAN WITHOUT SNOW                             |
| ⛅   | `\u{26C5}` | SUN BEHIND CLOUD                                 |
| ⛆    | `\u{26C6}` | RAIN                                             |
| ⛇    | `\u{26C7}` | BLACK SNOWMAN                                    |
| ⛈    | `\u{26C8}` | THUNDER CLOUD AND RAIN                           |
| ⛉    | `\u{26C9}` | TURNED WHITE SHOGI PIECE                         |
| ⛊    | `\u{26CA}` | TURNED BLACK SHOGI PIECE                         |
| ⛋    | `\u{26CB}` | WHITE DIAMOND IN SQUARE                          |
| ⛌    | `\u{26CC}` | CROSSING LANES                                   |
| ⛍    | `\u{26CD}` | DISABLED CAR                                     |
| ⛎   | `\u{26CE}` | OPHIUCHUS                                        |
| ⛏    | `\u{26CF}` | PICK                                             |
| ⛐    | `\u{26D0}` | CAR SLIDING                                      |
| ⛑    | `\u{26D1}` | HELMET WITH WHITE CROSS                          |
| ⛒    | `\u{26D2}` | CIRCLED CROSSING LANES                           |
| ⛓    | `\u{26D3}` | CHAINS                                           |
| ⛔   | `\u{26D4}` | NO ENTRY                                         |
| ⛕    | `\u{26D5}` | ALTERNATE ONE-WAY LEFT WAY TRAFFIC               |
| ⛖    | `\u{26D6}` | BLACK TWO-WAY LEFT WAY TRAFFIC                   |
| ⛗    | `\u{26D7}` | WHITE TWO-WAY LEFT WAY TRAFFIC                   |
| ⛘    | `\u{26D8}` | BLACK LEFT LANE MERGE                            |
| ⛙    | `\u{26D9}` | WHITE LEFT LANE MERGE                            |
| ⛚    | `\u{26DA}` | DRIVE SLOW SIGN                                  |
| ⛛    | `\u{26DB}` | HEAVY WHITE DOWN-POINTING TRIANGLE               |
| ⛜    | `\u{26DC}` | LEFT CLOSED ENTRY                                |
| ⛝    | `\u{26DD}` | SQUARED SALTIRE                                  |
| ⛞    | `\u{26DE}` | FALLING DIAGONAL IN WHITE CIRCLE IN BLACK SQUARE |
| ⛟    | `\u{26DF}` | BLACK TRUCK                                      |
| ⛠    | `\u{26E0}` | RESTRICTED LEFT ENTRY-1                          |
| ⛡    | `\u{26E1}` | RESTRICTED LEFT ENTRY-2                          |
| ⛢    | `\u{26E2}` | ASTRONOMICAL SYMBOL FOR URANUS                   |
| ⛣    | `\u{26E3}` | HEAVY CIRCLE WITH STROKE AND TWO DOTS ABOVE      |
| ⛤    | `\u{26E4}` | PENTAGRAM                                        |
| ⛥    | `\u{26E5}` | RIGHT-HANDED INTERLACED PENTAGRAM                |
| ⛦    | `\u{26E6}` | LEFT-HANDED INTERLACED PENTAGRAM                 |
| ⛧    | `\u{26E7}` | INVERTED PENTAGRAM                               |
| ⛨    | `\u{26E8}` | BLACK CROSS ON SHIELD                            |
| ⛩    | `\u{26E9}` | SHINTO SHRINE                                    |
| ⛪   | `\u{26EA}` | CHURCH                                           |
| ⛫    | `\u{26EB}` | CASTLE                                           |
| ⛬    | `\u{26EC}` | HISTORIC SITE                                    |
| ⛭    | `\u{26ED}` | GEAR WITHOUT HUB                                 |
| ⛮    | `\u{26EE}` | GEAR WITH HANDLES                                |
| ⛯    | `\u{26EF}` | MAP SYMBOL FOR LIGHTHOUSE                        |
| ⛰    | `\u{26F0}` | MOUNTAIN                                         |
| ⛱    | `\u{26F1}` | UMBRELLA ON GROUND                               |
| ⛲   | `\u{26F2}` | FOUNTAIN                                         |
| ⛳   | `\u{26F3}` | FLAG IN HOLE                                     |
| ⛴    | `\u{26F4}` | FERRY                                            |
| ⛵   | `\u{26F5}` | SAILBOAT                                         |
| ⛶    | `\u{26F6}` | SQUARE FOUR CORNERS                              |
| ⛷    | `\u{26F7}` | SKIER                                            |
| ⛸    | `\u{26F8}` | ICE SKATE                                        |
| ⛹    | `\u{26F9}` | PERSON WITH BALL                                 |
| ⛺   | `\u{26FA}` | TENT                                             |
| ⛻    | `\u{26FB}` | JAPANESE BANK SYMBOL                             |
| ⛼    | `\u{26FC}` | HEADSTONE GRAVEYARD SYMBOL                       |
| ⛽   | `\u{26FD}` | FUEL PUMP                                        |
| ⛾    | `\u{26FE}` | CUP ON BLACK SQUARE                              |
| ⛿    | `\u{26FF}` | WHITE FLAG WITH HORIZONTAL MIDDLE BLACK STRIPE   |

## Miscellaneous Symbols and Arrows (256)

| Char | Escape     | Name                                                                       |
|------|------------|----------------------------------------------------------------------------|
| ⬀    | `\u{2B00}` | NORTH EAST WHITE ARROW                                                     |
| ⬁    | `\u{2B01}` | NORTH WEST WHITE ARROW                                                     |
| ⬂    | `\u{2B02}` | SOUTH EAST WHITE ARROW                                                     |
| ⬃    | `\u{2B03}` | SOUTH WEST WHITE ARROW                                                     |
| ⬄    | `\u{2B04}` | LEFT RIGHT WHITE ARROW                                                     |
| ⬅    | `\u{2B05}` | LEFTWARDS BLACK ARROW                                                      |
| ⬆    | `\u{2B06}` | UPWARDS BLACK ARROW                                                        |
| ⬇    | `\u{2B07}` | DOWNWARDS BLACK ARROW                                                      |
| ⬈    | `\u{2B08}` | NORTH EAST BLACK ARROW                                                     |
| ⬉    | `\u{2B09}` | NORTH WEST BLACK ARROW                                                     |
| ⬊    | `\u{2B0A}` | SOUTH EAST BLACK ARROW                                                     |
| ⬋    | `\u{2B0B}` | SOUTH WEST BLACK ARROW                                                     |
| ⬌    | `\u{2B0C}` | LEFT RIGHT BLACK ARROW                                                     |
| ⬍    | `\u{2B0D}` | UP DOWN BLACK ARROW                                                        |
| ⬎    | `\u{2B0E}` | RIGHTWARDS ARROW WITH TIP DOWNWARDS                                        |
| ⬏    | `\u{2B0F}` | RIGHTWARDS ARROW WITH TIP UPWARDS                                          |
| ⬐    | `\u{2B10}` | LEFTWARDS ARROW WITH TIP DOWNWARDS                                         |
| ⬑    | `\u{2B11}` | LEFTWARDS ARROW WITH TIP UPWARDS                                           |
| ⬒    | `\u{2B12}` | SQUARE WITH TOP HALF BLACK                                                 |
| ⬓    | `\u{2B13}` | SQUARE WITH BOTTOM HALF BLACK                                              |
| ⬔    | `\u{2B14}` | SQUARE WITH UPPER RIGHT DIAGONAL HALF BLACK                                |
| ⬕    | `\u{2B15}` | SQUARE WITH LOWER LEFT DIAGONAL HALF BLACK                                 |
| ⬖    | `\u{2B16}` | DIAMOND WITH LEFT HALF BLACK                                               |
| ⬗    | `\u{2B17}` | DIAMOND WITH RIGHT HALF BLACK                                              |
| ⬘    | `\u{2B18}` | DIAMOND WITH TOP HALF BLACK                                                |
| ⬙    | `\u{2B19}` | DIAMOND WITH BOTTOM HALF BLACK                                             |
| ⬚    | `\u{2B1A}` | DOTTED SQUARE                                                              |
| ⬛   | `\u{2B1B}` | BLACK LARGE SQUARE                                                         |
| ⬜   | `\u{2B1C}` | WHITE LARGE SQUARE                                                         |
| ⬝    | `\u{2B1D}` | BLACK VERY SMALL SQUARE                                                    |
| ⬞    | `\u{2B1E}` | WHITE VERY SMALL SQUARE                                                    |
| ⬟    | `\u{2B1F}` | BLACK PENTAGON                                                             |
| ⬠    | `\u{2B20}` | WHITE PENTAGON                                                             |
| ⬡    | `\u{2B21}` | WHITE HEXAGON                                                              |
| ⬢    | `\u{2B22}` | BLACK HEXAGON                                                              |
| ⬣    | `\u{2B23}` | HORIZONTAL BLACK HEXAGON                                                   |
| ⬤    | `\u{2B24}` | BLACK LARGE CIRCLE                                                         |
| ⬥    | `\u{2B25}` | BLACK MEDIUM DIAMOND                                                       |
| ⬦    | `\u{2B26}` | WHITE MEDIUM DIAMOND                                                       |
| ⬧    | `\u{2B27}` | BLACK MEDIUM LOZENGE                                                       |
| ⬨    | `\u{2B28}` | WHITE MEDIUM LOZENGE                                                       |
| ⬩    | `\u{2B29}` | BLACK SMALL DIAMOND                                                        |
| ⬪    | `\u{2B2A}` | BLACK SMALL LOZENGE                                                        |
| ⬫    | `\u{2B2B}` | WHITE SMALL LOZENGE                                                        |
| ⬬    | `\u{2B2C}` | BLACK HORIZONTAL ELLIPSE                                                   |
| ⬭    | `\u{2B2D}` | WHITE HORIZONTAL ELLIPSE                                                   |
| ⬮    | `\u{2B2E}` | BLACK VERTICAL ELLIPSE                                                     |
| ⬯    | `\u{2B2F}` | WHITE VERTICAL ELLIPSE                                                     |
| ⬰    | `\u{2B30}` | LEFT ARROW WITH SMALL CIRCLE                                               |
| ⬱    | `\u{2B31}` | THREE LEFTWARDS ARROWS                                                     |
| ⬲    | `\u{2B32}` | LEFT ARROW WITH CIRCLED PLUS                                               |
| ⬳    | `\u{2B33}` | LONG LEFTWARDS SQUIGGLE ARROW                                              |
| ⬴    | `\u{2B34}` | LEFTWARDS TWO-HEADED ARROW WITH VERTICAL STROKE                            |
| ⬵    | `\u{2B35}` | LEFTWARDS TWO-HEADED ARROW WITH DOUBLE VERTICAL STROKE                     |
| ⬶    | `\u{2B36}` | LEFTWARDS TWO-HEADED ARROW FROM BAR                                        |
| ⬷    | `\u{2B37}` | LEFTWARDS TWO-HEADED TRIPLE DASH ARROW                                     |
| ⬸    | `\u{2B38}` | LEFTWARDS ARROW WITH DOTTED STEM                                           |
| ⬹    | `\u{2B39}` | LEFTWARDS ARROW WITH TAIL WITH VERTICAL STROKE                             |
| ⬺    | `\u{2B3A}` | LEFTWARDS ARROW WITH TAIL WITH DOUBLE VERTICAL STROKE                      |
| ⬻    | `\u{2B3B}` | LEFTWARDS TWO-HEADED ARROW WITH TAIL                                       |
| ⬼    | `\u{2B3C}` | LEFTWARDS TWO-HEADED ARROW WITH TAIL WITH VERTICAL STROKE                  |
| ⬽    | `\u{2B3D}` | LEFTWARDS TWO-HEADED ARROW WITH TAIL WITH DOUBLE VERTICAL STROKE           |
| ⬾    | `\u{2B3E}` | LEFTWARDS ARROW THROUGH X                                                  |
| ⬿    | `\u{2B3F}` | WAVE ARROW POINTING DIRECTLY LEFT                                          |
| ⭀    | `\u{2B40}` | EQUALS SIGN ABOVE LEFTWARDS ARROW                                          |
| ⭁    | `\u{2B41}` | REVERSE TILDE OPERATOR ABOVE LEFTWARDS ARROW                               |
| ⭂    | `\u{2B42}` | LEFTWARDS ARROW ABOVE REVERSE ALMOST EQUAL TO                              |
| ⭃    | `\u{2B43}` | RIGHTWARDS ARROW THROUGH GREATER-THAN                                      |
| ⭄    | `\u{2B44}` | RIGHTWARDS ARROW THROUGH SUPERSET                                          |
| ⭅    | `\u{2B45}` | LEFTWARDS QUADRUPLE ARROW                                                  |
| ⭆    | `\u{2B46}` | RIGHTWARDS QUADRUPLE ARROW                                                 |
| ⭇    | `\u{2B47}` | REVERSE TILDE OPERATOR ABOVE RIGHTWARDS ARROW                              |
| ⭈    | `\u{2B48}` | RIGHTWARDS ARROW ABOVE REVERSE ALMOST EQUAL TO                             |
| ⭉    | `\u{2B49}` | TILDE OPERATOR ABOVE LEFTWARDS ARROW                                       |
| ⭊    | `\u{2B4A}` | LEFTWARDS ARROW ABOVE ALMOST EQUAL TO                                      |
| ⭋    | `\u{2B4B}` | LEFTWARDS ARROW ABOVE REVERSE TILDE OPERATOR                               |
| ⭌    | `\u{2B4C}` | RIGHTWARDS ARROW ABOVE REVERSE TILDE OPERATOR                              |
| ⭍    | `\u{2B4D}` | DOWNWARDS TRIANGLE-HEADED ZIGZAG ARROW                                     |
| ⭎    | `\u{2B4E}` | SHORT SLANTED NORTH ARROW                                                  |
| ⭏    | `\u{2B4F}` | SHORT BACKSLANTED SOUTH ARROW                                              |
| ⭐   | `\u{2B50}` | WHITE MEDIUM STAR                                                          |
| ⭑    | `\u{2B51}` | BLACK SMALL STAR                                                           |
| ⭒    | `\u{2B52}` | WHITE SMALL STAR                                                           |
| ⭓    | `\u{2B53}` | BLACK RIGHT-POINTING PENTAGON                                              |
| ⭔    | `\u{2B54}` | WHITE RIGHT-POINTING PENTAGON                                              |
| ⭕   | `\u{2B55}` | HEAVY LARGE CIRCLE                                                         |
| ⭖    | `\u{2B56}` | HEAVY OVAL WITH OVAL INSIDE                                                |
| ⭗    | `\u{2B57}` | HEAVY CIRCLE WITH CIRCLE INSIDE                                            |
| ⭘    | `\u{2B58}` | HEAVY CIRCLE                                                               |
| ⭙    | `\u{2B59}` | HEAVY CIRCLED SALTIRE                                                      |
| ⭚    | `\u{2B5A}` | SLANTED NORTH ARROW WITH HOOKED HEAD                                       |
| ⭛    | `\u{2B5B}` | BACKSLANTED SOUTH ARROW WITH HOOKED TAIL                                   |
| ⭜    | `\u{2B5C}` | SLANTED NORTH ARROW WITH HORIZONTAL TAIL                                   |
| ⭝    | `\u{2B5D}` | BACKSLANTED SOUTH ARROW WITH HORIZONTAL TAIL                               |
| ⭞    | `\u{2B5E}` | BENT ARROW POINTING DOWNWARDS THEN NORTH EAST                              |
| ⭟    | `\u{2B5F}` | SHORT BENT ARROW POINTING DOWNWARDS THEN NORTH EAST                        |
| ⭠    | `\u{2B60}` | LEFTWARDS TRIANGLE-HEADED ARROW                                            |
| ⭡    | `\u{2B61}` | UPWARDS TRIANGLE-HEADED ARROW                                              |
| ⭢    | `\u{2B62}` | RIGHTWARDS TRIANGLE-HEADED ARROW                                           |
| ⭣    | `\u{2B63}` | DOWNWARDS TRIANGLE-HEADED ARROW                                            |
| ⭤    | `\u{2B64}` | LEFT RIGHT TRIANGLE-HEADED ARROW                                           |
| ⭥    | `\u{2B65}` | UP DOWN TRIANGLE-HEADED ARROW                                              |
| ⭦    | `\u{2B66}` | NORTH WEST TRIANGLE-HEADED ARROW                                           |
| ⭧    | `\u{2B67}` | NORTH EAST TRIANGLE-HEADED ARROW                                           |
| ⭨    | `\u{2B68}` | SOUTH EAST TRIANGLE-HEADED ARROW                                           |
| ⭩    | `\u{2B69}` | SOUTH WEST TRIANGLE-HEADED ARROW                                           |
| ⭪    | `\u{2B6A}` | LEFTWARDS TRIANGLE-HEADED DASHED ARROW                                     |
| ⭫    | `\u{2B6B}` | UPWARDS TRIANGLE-HEADED DASHED ARROW                                       |
| ⭬    | `\u{2B6C}` | RIGHTWARDS TRIANGLE-HEADED DASHED ARROW                                    |
| ⭭    | `\u{2B6D}` | DOWNWARDS TRIANGLE-HEADED DASHED ARROW                                     |
| ⭮    | `\u{2B6E}` | CLOCKWISE TRIANGLE-HEADED OPEN CIRCLE ARROW                                |
| ⭯    | `\u{2B6F}` | ANTICLOCKWISE TRIANGLE-HEADED OPEN CIRCLE ARROW                            |
| ⭰    | `\u{2B70}` | LEFTWARDS TRIANGLE-HEADED ARROW TO BAR                                     |
| ⭱    | `\u{2B71}` | UPWARDS TRIANGLE-HEADED ARROW TO BAR                                       |
| ⭲    | `\u{2B72}` | RIGHTWARDS TRIANGLE-HEADED ARROW TO BAR                                    |
| ⭳    | `\u{2B73}` | DOWNWARDS TRIANGLE-HEADED ARROW TO BAR                                     |
| ⭴    | `\u{2B74}` |                                                                            |
| ⭵    | `\u{2B75}` |                                                                            |
| ⭶    | `\u{2B76}` | NORTH WEST TRIANGLE-HEADED ARROW TO BAR                                    |
| ⭷    | `\u{2B77}` | NORTH EAST TRIANGLE-HEADED ARROW TO BAR                                    |
| ⭸    | `\u{2B78}` | SOUTH EAST TRIANGLE-HEADED ARROW TO BAR                                    |
| ⭹    | `\u{2B79}` | SOUTH WEST TRIANGLE-HEADED ARROW TO BAR                                    |
| ⭺    | `\u{2B7A}` | LEFTWARDS TRIANGLE-HEADED ARROW WITH DOUBLE HORIZONTAL STROKE              |
| ⭻    | `\u{2B7B}` | UPWARDS TRIANGLE-HEADED ARROW WITH DOUBLE HORIZONTAL STROKE                |
| ⭼    | `\u{2B7C}` | RIGHTWARDS TRIANGLE-HEADED ARROW WITH DOUBLE HORIZONTAL STROKE             |
| ⭽    | `\u{2B7D}` | DOWNWARDS TRIANGLE-HEADED ARROW WITH DOUBLE HORIZONTAL STROKE              |
| ⭾    | `\u{2B7E}` | HORIZONTAL TAB KEY                                                         |
| ⭿    | `\u{2B7F}` | VERTICAL TAB KEY                                                           |
| ⮀    | `\u{2B80}` | LEFTWARDS TRIANGLE-HEADED ARROW OVER RIGHTWARDS TRIANGLE-HEADED ARROW      |
| ⮁    | `\u{2B81}` | UPWARDS TRIANGLE-HEADED ARROW LEFTWARDS OF DOWNWARDS TRIANGLE-HEADED ARROW |
| ⮂    | `\u{2B82}` | RIGHTWARDS TRIANGLE-HEADED ARROW OVER LEFTWARDS TRIANGLE-HEADED ARROW      |
| ⮃    | `\u{2B83}` | DOWNWARDS TRIANGLE-HEADED ARROW LEFTWARDS OF UPWARDS TRIANGLE-HEADED ARROW |
| ⮄    | `\u{2B84}` | LEFTWARDS TRIANGLE-HEADED PAIRED ARROWS                                    |
| ⮅    | `\u{2B85}` | UPWARDS TRIANGLE-HEADED PAIRED ARROWS                                      |
| ⮆    | `\u{2B86}` | RIGHTWARDS TRIANGLE-HEADED PAIRED ARROWS                                   |
| ⮇    | `\u{2B87}` | DOWNWARDS TRIANGLE-HEADED PAIRED ARROWS                                    |
| ⮈    | `\u{2B88}` | LEFTWARDS BLACK CIRCLED WHITE ARROW                                        |
| ⮉    | `\u{2B89}` | UPWARDS BLACK CIRCLED WHITE ARROW                                          |
| ⮊    | `\u{2B8A}` | RIGHTWARDS BLACK CIRCLED WHITE ARROW                                       |
| ⮋    | `\u{2B8B}` | DOWNWARDS BLACK CIRCLED WHITE ARROW                                        |
| ⮌    | `\u{2B8C}` | ANTICLOCKWISE TRIANGLE-HEADED RIGHT U-SHAPED ARROW                         |
| ⮍    | `\u{2B8D}` | ANTICLOCKWISE TRIANGLE-HEADED BOTTOM U-SHAPED ARROW                        |
| ⮎    | `\u{2B8E}` | ANTICLOCKWISE TRIANGLE-HEADED LEFT U-SHAPED ARROW                          |
| ⮏    | `\u{2B8F}` | ANTICLOCKWISE TRIANGLE-HEADED TOP U-SHAPED ARROW                           |
| ⮐    | `\u{2B90}` | RETURN LEFT                                                                |
| ⮑    | `\u{2B91}` | RETURN RIGHT                                                               |
| ⮒    | `\u{2B92}` | NEWLINE LEFT                                                               |
| ⮓    | `\u{2B93}` | NEWLINE RIGHT                                                              |
| ⮔    | `\u{2B94}` | FOUR CORNER ARROWS CIRCLING ANTICLOCKWISE                                  |
| ⮕    | `\u{2B95}` | RIGHTWARDS BLACK ARROW                                                     |
| ⮖    | `\u{2B96}` |                                                                            |
| ⮗    | `\u{2B97}` | SYMBOL FOR TYPE A ELECTRONICS                                              |
| ⮘    | `\u{2B98}` | THREE-D TOP-LIGHTED LEFTWARDS EQUILATERAL ARROWHEAD                        |
| ⮙    | `\u{2B99}` | THREE-D RIGHT-LIGHTED UPWARDS EQUILATERAL ARROWHEAD                        |
| ⮚    | `\u{2B9A}` | THREE-D TOP-LIGHTED RIGHTWARDS EQUILATERAL ARROWHEAD                       |
| ⮛    | `\u{2B9B}` | THREE-D LEFT-LIGHTED DOWNWARDS EQUILATERAL ARROWHEAD                       |
| ⮜    | `\u{2B9C}` | BLACK LEFTWARDS EQUILATERAL ARROWHEAD                                      |
| ⮝    | `\u{2B9D}` | BLACK UPWARDS EQUILATERAL ARROWHEAD                                        |
| ⮞    | `\u{2B9E}` | BLACK RIGHTWARDS EQUILATERAL ARROWHEAD                                     |
| ⮟    | `\u{2B9F}` | BLACK DOWNWARDS EQUILATERAL ARROWHEAD                                      |
| ⮠    | `\u{2BA0}` | DOWNWARDS TRIANGLE-HEADED ARROW WITH LONG TIP LEFTWARDS                    |
| ⮡    | `\u{2BA1}` | DOWNWARDS TRIANGLE-HEADED ARROW WITH LONG TIP RIGHTWARDS                   |
| ⮢    | `\u{2BA2}` | UPWARDS TRIANGLE-HEADED ARROW WITH LONG TIP LEFTWARDS                      |
| ⮣    | `\u{2BA3}` | UPWARDS TRIANGLE-HEADED ARROW WITH LONG TIP RIGHTWARDS                     |
| ⮤    | `\u{2BA4}` | LEFTWARDS TRIANGLE-HEADED ARROW WITH LONG TIP UPWARDS                      |
| ⮥    | `\u{2BA5}` | RIGHTWARDS TRIANGLE-HEADED ARROW WITH LONG TIP UPWARDS                     |
| ⮦    | `\u{2BA6}` | LEFTWARDS TRIANGLE-HEADED ARROW WITH LONG TIP DOWNWARDS                    |
| ⮧    | `\u{2BA7}` | RIGHTWARDS TRIANGLE-HEADED ARROW WITH LONG TIP DOWNWARDS                   |
| ⮨    | `\u{2BA8}` | BLACK CURVED DOWNWARDS AND LEFTWARDS ARROW                                 |
| ⮩    | `\u{2BA9}` | BLACK CURVED DOWNWARDS AND RIGHTWARDS ARROW                                |
| ⮪    | `\u{2BAA}` | BLACK CURVED UPWARDS AND LEFTWARDS ARROW                                   |
| ⮫    | `\u{2BAB}` | BLACK CURVED UPWARDS AND RIGHTWARDS ARROW                                  |
| ⮬    | `\u{2BAC}` | BLACK CURVED LEFTWARDS AND UPWARDS ARROW                                   |
| ⮭    | `\u{2BAD}` | BLACK CURVED RIGHTWARDS AND UPWARDS ARROW                                  |
| ⮮    | `\u{2BAE}` | BLACK CURVED LEFTWARDS AND DOWNWARDS ARROW                                 |
| ⮯    | `\u{2BAF}` | BLACK CURVED RIGHTWARDS AND DOWNWARDS ARROW                                |
| ⮰    | `\u{2BB0}` | RIBBON ARROW DOWN LEFT                                                     |
| ⮱    | `\u{2BB1}` | RIBBON ARROW DOWN RIGHT                                                    |
| ⮲    | `\u{2BB2}` | RIBBON ARROW UP LEFT                                                       |
| ⮳    | `\u{2BB3}` | RIBBON ARROW UP RIGHT                                                      |
| ⮴    | `\u{2BB4}` | RIBBON ARROW LEFT UP                                                       |
| ⮵    | `\u{2BB5}` | RIBBON ARROW RIGHT UP                                                      |
| ⮶    | `\u{2BB6}` | RIBBON ARROW LEFT DOWN                                                     |
| ⮷    | `\u{2BB7}` | RIBBON ARROW RIGHT DOWN                                                    |
| ⮸    | `\u{2BB8}` | UPWARDS WHITE ARROW FROM BAR WITH HORIZONTAL BAR                           |
| ⮹    | `\u{2BB9}` | UP ARROWHEAD IN A RECTANGLE BOX                                            |
| ⮺    | `\u{2BBA}` | OVERLAPPING WHITE SQUARES                                                  |
| ⮻    | `\u{2BBB}` | OVERLAPPING WHITE AND BLACK SQUARES                                        |
| ⮼    | `\u{2BBC}` | OVERLAPPING BLACK SQUARES                                                  |
| ⮽    | `\u{2BBD}` | BALLOT BOX WITH LIGHT X                                                    |
| ⮾    | `\u{2BBE}` | CIRCLED X                                                                  |
| ⮿    | `\u{2BBF}` | CIRCLED BOLD X                                                             |
| ⯀    | `\u{2BC0}` | BLACK SQUARE CENTRED                                                       |
| ⯁    | `\u{2BC1}` | BLACK DIAMOND CENTRED                                                      |
| ⯂    | `\u{2BC2}` | TURNED BLACK PENTAGON                                                      |
| ⯃    | `\u{2BC3}` | HORIZONTAL BLACK OCTAGON                                                   |
| ⯄    | `\u{2BC4}` | BLACK OCTAGON                                                              |
| ⯅    | `\u{2BC5}` | BLACK MEDIUM UP-POINTING TRIANGLE CENTRED                                  |
| ⯆    | `\u{2BC6}` | BLACK MEDIUM DOWN-POINTING TRIANGLE CENTRED                                |
| ⯇    | `\u{2BC7}` | BLACK MEDIUM LEFT-POINTING TRIANGLE CENTRED                                |
| ⯈    | `\u{2BC8}` | BLACK MEDIUM RIGHT-POINTING TRIANGLE CENTRED                               |
| ⯉    | `\u{2BC9}` | NEPTUNE FORM TWO                                                           |
| ⯊    | `\u{2BCA}` | TOP HALF BLACK CIRCLE                                                      |
| ⯋    | `\u{2BCB}` | BOTTOM HALF BLACK CIRCLE                                                   |
| ⯌    | `\u{2BCC}` | LIGHT FOUR POINTED BLACK CUSP                                              |
| ⯍    | `\u{2BCD}` | ROTATED LIGHT FOUR POINTED BLACK CUSP                                      |
| ⯎    | `\u{2BCE}` | WHITE FOUR POINTED CUSP                                                    |
| ⯏    | `\u{2BCF}` | ROTATED WHITE FOUR POINTED CUSP                                            |
| ⯐    | `\u{2BD0}` | SQUARE POSITION INDICATOR                                                  |
| ⯑    | `\u{2BD1}` | UNCERTAINTY SIGN                                                           |
| ⯒    | `\u{2BD2}` | GROUP MARK                                                                 |
| ⯓    | `\u{2BD3}` | PLUTO FORM TWO                                                             |
| ⯔    | `\u{2BD4}` | PLUTO FORM THREE                                                           |
| ⯕    | `\u{2BD5}` | PLUTO FORM FOUR                                                            |
| ⯖    | `\u{2BD6}` | PLUTO FORM FIVE                                                            |
| ⯗    | `\u{2BD7}` | TRANSPLUTO                                                                 |
| ⯘    | `\u{2BD8}` | PROSERPINA                                                                 |
| ⯙    | `\u{2BD9}` | ASTRAEA                                                                    |
| ⯚    | `\u{2BDA}` | HYGIEA                                                                     |
| ⯛    | `\u{2BDB}` | PHOLUS                                                                     |
| ⯜    | `\u{2BDC}` | NESSUS                                                                     |
| ⯝    | `\u{2BDD}` | WHITE MOON SELENA                                                          |
| ⯞    | `\u{2BDE}` | BLACK DIAMOND ON CROSS                                                     |
| ⯟    | `\u{2BDF}` | TRUE LIGHT MOON ARTA                                                       |
| ⯠    | `\u{2BE0}` | CUPIDO                                                                     |
| ⯡    | `\u{2BE1}` | HADES                                                                      |
| ⯢    | `\u{2BE2}` | ZEUS                                                                       |
| ⯣    | `\u{2BE3}` | KRONOS                                                                     |
| ⯤    | `\u{2BE4}` | APOLLON                                                                    |
| ⯥    | `\u{2BE5}` | ADMETOS                                                                    |
| ⯦    | `\u{2BE6}` | VULCANUS                                                                   |
| ⯧    | `\u{2BE7}` | POSEIDON                                                                   |
| ⯨    | `\u{2BE8}` | LEFT HALF BLACK STAR                                                       |
| ⯩    | `\u{2BE9}` | RIGHT HALF BLACK STAR                                                      |
| ⯪    | `\u{2BEA}` | STAR WITH LEFT HALF BLACK                                                  |
| ⯫    | `\u{2BEB}` | STAR WITH RIGHT HALF BLACK                                                 |
| ⯬    | `\u{2BEC}` | LEFTWARDS TWO-HEADED ARROW WITH TRIANGLE ARROWHEADS                        |
| ⯭    | `\u{2BED}` | UPWARDS TWO-HEADED ARROW WITH TRIANGLE ARROWHEADS                          |
| ⯮    | `\u{2BEE}` | RIGHTWARDS TWO-HEADED ARROW WITH TRIANGLE ARROWHEADS                       |
| ⯯    | `\u{2BEF}` | DOWNWARDS TWO-HEADED ARROW WITH TRIANGLE ARROWHEADS                        |
| ⯰    | `\u{2BF0}` | ERIS FORM ONE                                                              |
| ⯱    | `\u{2BF1}` | ERIS FORM TWO                                                              |
| ⯲    | `\u{2BF2}` | SEDNA                                                                      |
| ⯳    | `\u{2BF3}` | RUSSIAN ASTROLOGICAL SYMBOL VIGINTILE                                      |
| ⯴    | `\u{2BF4}` | RUSSIAN ASTROLOGICAL SYMBOL NOVILE                                         |
| ⯵    | `\u{2BF5}` | RUSSIAN ASTROLOGICAL SYMBOL QUINTILE                                       |
| ⯶    | `\u{2BF6}` | RUSSIAN ASTROLOGICAL SYMBOL BINOVILE                                       |
| ⯷    | `\u{2BF7}` | RUSSIAN ASTROLOGICAL SYMBOL SENTAGON                                       |
| ⯸    | `\u{2BF8}` | RUSSIAN ASTROLOGICAL SYMBOL TREDECILE                                      |
| ⯹    | `\u{2BF9}` | EQUALS SIGN WITH INFINITY BELOW                                            |
| ⯺    | `\u{2BFA}` | UNITED SYMBOL                                                              |
| ⯻    | `\u{2BFB}` | SEPARATED SYMBOL                                                           |
| ⯼    | `\u{2BFC}` | DOUBLED SYMBOL                                                             |
| ⯽    | `\u{2BFD}` | PASSED SYMBOL                                                              |
| ⯾    | `\u{2BFE}` | REVERSED RIGHT ANGLE                                                       |
| ⯿    | `\u{2BFF}` | HELLSCHREIBER PAUSE SYMBOL                                                 |

## Symbols for Legacy Computing (256)

| Char | Escape      | Name                                                                                     |
|------|-------------|------------------------------------------------------------------------------------------|
| 🬀    | `\u{1FB00}` | BLOCK SEXTANT-1                                                                          |
| 🬁    | `\u{1FB01}` | BLOCK SEXTANT-2                                                                          |
| 🬂    | `\u{1FB02}` | BLOCK SEXTANT-12                                                                         |
| 🬃    | `\u{1FB03}` | BLOCK SEXTANT-3                                                                          |
| 🬄    | `\u{1FB04}` | BLOCK SEXTANT-13                                                                         |
| 🬅    | `\u{1FB05}` | BLOCK SEXTANT-23                                                                         |
| 🬆    | `\u{1FB06}` | BLOCK SEXTANT-123                                                                        |
| 🬇    | `\u{1FB07}` | BLOCK SEXTANT-4                                                                          |
| 🬈    | `\u{1FB08}` | BLOCK SEXTANT-14                                                                         |
| 🬉    | `\u{1FB09}` | BLOCK SEXTANT-24                                                                         |
| 🬊    | `\u{1FB0A}` | BLOCK SEXTANT-124                                                                        |
| 🬋    | `\u{1FB0B}` | BLOCK SEXTANT-34                                                                         |
| 🬌    | `\u{1FB0C}` | BLOCK SEXTANT-134                                                                        |
| 🬍    | `\u{1FB0D}` | BLOCK SEXTANT-234                                                                        |
| 🬎    | `\u{1FB0E}` | BLOCK SEXTANT-1234                                                                       |
| 🬏    | `\u{1FB0F}` | BLOCK SEXTANT-5                                                                          |
| 🬐    | `\u{1FB10}` | BLOCK SEXTANT-15                                                                         |
| 🬑    | `\u{1FB11}` | BLOCK SEXTANT-25                                                                         |
| 🬒    | `\u{1FB12}` | BLOCK SEXTANT-125                                                                        |
| 🬓    | `\u{1FB13}` | BLOCK SEXTANT-35                                                                         |
| 🬔    | `\u{1FB14}` | BLOCK SEXTANT-235                                                                        |
| 🬕    | `\u{1FB15}` | BLOCK SEXTANT-1235                                                                       |
| 🬖    | `\u{1FB16}` | BLOCK SEXTANT-45                                                                         |
| 🬗    | `\u{1FB17}` | BLOCK SEXTANT-145                                                                        |
| 🬘    | `\u{1FB18}` | BLOCK SEXTANT-245                                                                        |
| 🬙    | `\u{1FB19}` | BLOCK SEXTANT-1245                                                                       |
| 🬚    | `\u{1FB1A}` | BLOCK SEXTANT-345                                                                        |
| 🬛    | `\u{1FB1B}` | BLOCK SEXTANT-1345                                                                       |
| 🬜    | `\u{1FB1C}` | BLOCK SEXTANT-2345                                                                       |
| 🬝    | `\u{1FB1D}` | BLOCK SEXTANT-12345                                                                      |
| 🬞    | `\u{1FB1E}` | BLOCK SEXTANT-6                                                                          |
| 🬟    | `\u{1FB1F}` | BLOCK SEXTANT-16                                                                         |
| 🬠    | `\u{1FB20}` | BLOCK SEXTANT-26                                                                         |
| 🬡    | `\u{1FB21}` | BLOCK SEXTANT-126                                                                        |
| 🬢    | `\u{1FB22}` | BLOCK SEXTANT-36                                                                         |
| 🬣    | `\u{1FB23}` | BLOCK SEXTANT-136                                                                        |
| 🬤    | `\u{1FB24}` | BLOCK SEXTANT-236                                                                        |
| 🬥    | `\u{1FB25}` | BLOCK SEXTANT-1236                                                                       |
| 🬦    | `\u{1FB26}` | BLOCK SEXTANT-46                                                                         |
| 🬧    | `\u{1FB27}` | BLOCK SEXTANT-146                                                                        |
| 🬨    | `\u{1FB28}` | BLOCK SEXTANT-1246                                                                       |
| 🬩    | `\u{1FB29}` | BLOCK SEXTANT-346                                                                        |
| 🬪    | `\u{1FB2A}` | BLOCK SEXTANT-1346                                                                       |
| 🬫    | `\u{1FB2B}` | BLOCK SEXTANT-2346                                                                       |
| 🬬    | `\u{1FB2C}` | BLOCK SEXTANT-12346                                                                      |
| 🬭    | `\u{1FB2D}` | BLOCK SEXTANT-56                                                                         |
| 🬮    | `\u{1FB2E}` | BLOCK SEXTANT-156                                                                        |
| 🬯    | `\u{1FB2F}` | BLOCK SEXTANT-256                                                                        |
| 🬰    | `\u{1FB30}` | BLOCK SEXTANT-1256                                                                       |
| 🬱    | `\u{1FB31}` | BLOCK SEXTANT-356                                                                        |
| 🬲    | `\u{1FB32}` | BLOCK SEXTANT-1356                                                                       |
| 🬳    | `\u{1FB33}` | BLOCK SEXTANT-2356                                                                       |
| 🬴    | `\u{1FB34}` | BLOCK SEXTANT-12356                                                                      |
| 🬵    | `\u{1FB35}` | BLOCK SEXTANT-456                                                                        |
| 🬶    | `\u{1FB36}` | BLOCK SEXTANT-1456                                                                       |
| 🬷    | `\u{1FB37}` | BLOCK SEXTANT-2456                                                                       |
| 🬸    | `\u{1FB38}` | BLOCK SEXTANT-12456                                                                      |
| 🬹    | `\u{1FB39}` | BLOCK SEXTANT-3456                                                                       |
| 🬺    | `\u{1FB3A}` | BLOCK SEXTANT-13456                                                                      |
| 🬻    | `\u{1FB3B}` | BLOCK SEXTANT-23456                                                                      |
| 🬼    | `\u{1FB3C}` | LOWER LEFT BLOCK DIAGONAL LOWER MIDDLE LEFT TO LOWER CENTRE                              |
| 🬽    | `\u{1FB3D}` | LOWER LEFT BLOCK DIAGONAL LOWER MIDDLE LEFT TO LOWER RIGHT                               |
| 🬾    | `\u{1FB3E}` | LOWER LEFT BLOCK DIAGONAL UPPER MIDDLE LEFT TO LOWER CENTRE                              |
| 🬿    | `\u{1FB3F}` | LOWER LEFT BLOCK DIAGONAL UPPER MIDDLE LEFT TO LOWER RIGHT                               |
| 🭀    | `\u{1FB40}` | LOWER LEFT BLOCK DIAGONAL UPPER LEFT TO LOWER CENTRE                                     |
| 🭁    | `\u{1FB41}` | LOWER RIGHT BLOCK DIAGONAL UPPER MIDDLE LEFT TO UPPER CENTRE                             |
| 🭂    | `\u{1FB42}` | LOWER RIGHT BLOCK DIAGONAL UPPER MIDDLE LEFT TO UPPER RIGHT                              |
| 🭃    | `\u{1FB43}` | LOWER RIGHT BLOCK DIAGONAL LOWER MIDDLE LEFT TO UPPER CENTRE                             |
| 🭄    | `\u{1FB44}` | LOWER RIGHT BLOCK DIAGONAL LOWER MIDDLE LEFT TO UPPER RIGHT                              |
| 🭅    | `\u{1FB45}` | LOWER RIGHT BLOCK DIAGONAL LOWER LEFT TO UPPER CENTRE                                    |
| 🭆    | `\u{1FB46}` | LOWER RIGHT BLOCK DIAGONAL LOWER MIDDLE LEFT TO UPPER MIDDLE RIGHT                       |
| 🭇    | `\u{1FB47}` | LOWER RIGHT BLOCK DIAGONAL LOWER CENTRE TO LOWER MIDDLE RIGHT                            |
| 🭈    | `\u{1FB48}` | LOWER RIGHT BLOCK DIAGONAL LOWER LEFT TO LOWER MIDDLE RIGHT                              |
| 🭉    | `\u{1FB49}` | LOWER RIGHT BLOCK DIAGONAL LOWER CENTRE TO UPPER MIDDLE RIGHT                            |
| 🭊    | `\u{1FB4A}` | LOWER RIGHT BLOCK DIAGONAL LOWER LEFT TO UPPER MIDDLE RIGHT                              |
| 🭋    | `\u{1FB4B}` | LOWER RIGHT BLOCK DIAGONAL LOWER CENTRE TO UPPER RIGHT                                   |
| 🭌    | `\u{1FB4C}` | LOWER LEFT BLOCK DIAGONAL UPPER CENTRE TO UPPER MIDDLE RIGHT                             |
| 🭍    | `\u{1FB4D}` | LOWER LEFT BLOCK DIAGONAL UPPER LEFT TO UPPER MIDDLE RIGHT                               |
| 🭎    | `\u{1FB4E}` | LOWER LEFT BLOCK DIAGONAL UPPER CENTRE TO LOWER MIDDLE RIGHT                             |
| 🭏    | `\u{1FB4F}` | LOWER LEFT BLOCK DIAGONAL UPPER LEFT TO LOWER MIDDLE RIGHT                               |
| 🭐    | `\u{1FB50}` | LOWER LEFT BLOCK DIAGONAL UPPER CENTRE TO LOWER RIGHT                                    |
| 🭑    | `\u{1FB51}` | LOWER LEFT BLOCK DIAGONAL UPPER MIDDLE LEFT TO LOWER MIDDLE RIGHT                        |
| 🭒    | `\u{1FB52}` | UPPER RIGHT BLOCK DIAGONAL LOWER MIDDLE LEFT TO LOWER CENTRE                             |
| 🭓    | `\u{1FB53}` | UPPER RIGHT BLOCK DIAGONAL LOWER MIDDLE LEFT TO LOWER RIGHT                              |
| 🭔    | `\u{1FB54}` | UPPER RIGHT BLOCK DIAGONAL UPPER MIDDLE LEFT TO LOWER CENTRE                             |
| 🭕    | `\u{1FB55}` | UPPER RIGHT BLOCK DIAGONAL UPPER MIDDLE LEFT TO LOWER RIGHT                              |
| 🭖    | `\u{1FB56}` | UPPER RIGHT BLOCK DIAGONAL UPPER LEFT TO LOWER CENTRE                                    |
| 🭗    | `\u{1FB57}` | UPPER LEFT BLOCK DIAGONAL UPPER MIDDLE LEFT TO UPPER CENTRE                              |
| 🭘    | `\u{1FB58}` | UPPER LEFT BLOCK DIAGONAL UPPER MIDDLE LEFT TO UPPER RIGHT                               |
| 🭙    | `\u{1FB59}` | UPPER LEFT BLOCK DIAGONAL LOWER MIDDLE LEFT TO UPPER CENTRE                              |
| 🭚    | `\u{1FB5A}` | UPPER LEFT BLOCK DIAGONAL LOWER MIDDLE LEFT TO UPPER RIGHT                               |
| 🭛    | `\u{1FB5B}` | UPPER LEFT BLOCK DIAGONAL LOWER LEFT TO UPPER CENTRE                                     |
| 🭜    | `\u{1FB5C}` | UPPER LEFT BLOCK DIAGONAL LOWER MIDDLE LEFT TO UPPER MIDDLE RIGHT                        |
| 🭝    | `\u{1FB5D}` | UPPER LEFT BLOCK DIAGONAL LOWER CENTRE TO LOWER MIDDLE RIGHT                             |
| 🭞    | `\u{1FB5E}` | UPPER LEFT BLOCK DIAGONAL LOWER LEFT TO LOWER MIDDLE RIGHT                               |
| 🭟    | `\u{1FB5F}` | UPPER LEFT BLOCK DIAGONAL LOWER CENTRE TO UPPER MIDDLE RIGHT                             |
| 🭠    | `\u{1FB60}` | UPPER LEFT BLOCK DIAGONAL LOWER LEFT TO UPPER MIDDLE RIGHT                               |
| 🭡    | `\u{1FB61}` | UPPER LEFT BLOCK DIAGONAL LOWER CENTRE TO UPPER RIGHT                                    |
| 🭢    | `\u{1FB62}` | UPPER RIGHT BLOCK DIAGONAL UPPER CENTRE TO UPPER MIDDLE RIGHT                            |
| 🭣    | `\u{1FB63}` | UPPER RIGHT BLOCK DIAGONAL UPPER LEFT TO UPPER MIDDLE RIGHT                              |
| 🭤    | `\u{1FB64}` | UPPER RIGHT BLOCK DIAGONAL UPPER CENTRE TO LOWER MIDDLE RIGHT                            |
| 🭥    | `\u{1FB65}` | UPPER RIGHT BLOCK DIAGONAL UPPER LEFT TO LOWER MIDDLE RIGHT                              |
| 🭦    | `\u{1FB66}` | UPPER RIGHT BLOCK DIAGONAL UPPER CENTRE TO LOWER RIGHT                                   |
| 🭧    | `\u{1FB67}` | UPPER RIGHT BLOCK DIAGONAL UPPER MIDDLE LEFT TO LOWER MIDDLE RIGHT                       |
| 🭨    | `\u{1FB68}` | UPPER AND RIGHT AND LOWER TRIANGULAR THREE QUARTERS BLOCK                                |
| 🭩    | `\u{1FB69}` | LEFT AND LOWER AND RIGHT TRIANGULAR THREE QUARTERS BLOCK                                 |
| 🭪    | `\u{1FB6A}` | UPPER AND LEFT AND LOWER TRIANGULAR THREE QUARTERS BLOCK                                 |
| 🭫    | `\u{1FB6B}` | LEFT AND UPPER AND RIGHT TRIANGULAR THREE QUARTERS BLOCK                                 |
| 🭬    | `\u{1FB6C}` | LEFT TRIANGULAR ONE QUARTER BLOCK                                                        |
| 🭭    | `\u{1FB6D}` | UPPER TRIANGULAR ONE QUARTER BLOCK                                                       |
| 🭮    | `\u{1FB6E}` | RIGHT TRIANGULAR ONE QUARTER BLOCK                                                       |
| 🭯    | `\u{1FB6F}` | LOWER TRIANGULAR ONE QUARTER BLOCK                                                       |
| 🭰    | `\u{1FB70}` | VERTICAL ONE EIGHTH BLOCK-2                                                              |
| 🭱    | `\u{1FB71}` | VERTICAL ONE EIGHTH BLOCK-3                                                              |
| 🭲    | `\u{1FB72}` | VERTICAL ONE EIGHTH BLOCK-4                                                              |
| 🭳    | `\u{1FB73}` | VERTICAL ONE EIGHTH BLOCK-5                                                              |
| 🭴    | `\u{1FB74}` | VERTICAL ONE EIGHTH BLOCK-6                                                              |
| 🭵    | `\u{1FB75}` | VERTICAL ONE EIGHTH BLOCK-7                                                              |
| 🭶    | `\u{1FB76}` | HORIZONTAL ONE EIGHTH BLOCK-2                                                            |
| 🭷    | `\u{1FB77}` | HORIZONTAL ONE EIGHTH BLOCK-3                                                            |
| 🭸    | `\u{1FB78}` | HORIZONTAL ONE EIGHTH BLOCK-4                                                            |
| 🭹    | `\u{1FB79}` | HORIZONTAL ONE EIGHTH BLOCK-5                                                            |
| 🭺    | `\u{1FB7A}` | HORIZONTAL ONE EIGHTH BLOCK-6                                                            |
| 🭻    | `\u{1FB7B}` | HORIZONTAL ONE EIGHTH BLOCK-7                                                            |
| 🭼    | `\u{1FB7C}` | LEFT AND LOWER ONE EIGHTH BLOCK                                                          |
| 🭽    | `\u{1FB7D}` | LEFT AND UPPER ONE EIGHTH BLOCK                                                          |
| 🭾    | `\u{1FB7E}` | RIGHT AND UPPER ONE EIGHTH BLOCK                                                         |
| 🭿    | `\u{1FB7F}` | RIGHT AND LOWER ONE EIGHTH BLOCK                                                         |
| 🮀    | `\u{1FB80}` | UPPER AND LOWER ONE EIGHTH BLOCK                                                         |
| 🮁    | `\u{1FB81}` | HORIZONTAL ONE EIGHTH BLOCK-1358                                                         |
| 🮂    | `\u{1FB82}` | UPPER ONE QUARTER BLOCK                                                                  |
| 🮃    | `\u{1FB83}` | UPPER THREE EIGHTHS BLOCK                                                                |
| 🮄    | `\u{1FB84}` | UPPER FIVE EIGHTHS BLOCK                                                                 |
| 🮅    | `\u{1FB85}` | UPPER THREE QUARTERS BLOCK                                                               |
| 🮆    | `\u{1FB86}` | UPPER SEVEN EIGHTHS BLOCK                                                                |
| 🮇    | `\u{1FB87}` | RIGHT ONE QUARTER BLOCK                                                                  |
| 🮈    | `\u{1FB88}` | RIGHT THREE EIGHTHS BLOCK                                                                |
| 🮉    | `\u{1FB89}` | RIGHT FIVE EIGHTHS BLOCK                                                                 |
| 🮊    | `\u{1FB8A}` | RIGHT THREE QUARTERS BLOCK                                                               |
| 🮋    | `\u{1FB8B}` | RIGHT SEVEN EIGHTHS BLOCK                                                                |
| 🮌    | `\u{1FB8C}` | LEFT HALF MEDIUM SHADE                                                                   |
| 🮍    | `\u{1FB8D}` | RIGHT HALF MEDIUM SHADE                                                                  |
| 🮎    | `\u{1FB8E}` | UPPER HALF MEDIUM SHADE                                                                  |
| 🮏    | `\u{1FB8F}` | LOWER HALF MEDIUM SHADE                                                                  |
| 🮐    | `\u{1FB90}` | INVERSE MEDIUM SHADE                                                                     |
| 🮑    | `\u{1FB91}` | UPPER HALF BLOCK AND LOWER HALF INVERSE MEDIUM SHADE                                     |
| 🮒    | `\u{1FB92}` | UPPER HALF INVERSE MEDIUM SHADE AND LOWER HALF BLOCK                                     |
| 🮓    | `\u{1FB93}` |                                                                                          |
| 🮔    | `\u{1FB94}` | LEFT HALF INVERSE MEDIUM SHADE AND RIGHT HALF BLOCK                                      |
| 🮕    | `\u{1FB95}` | CHECKER BOARD FILL                                                                       |
| 🮖    | `\u{1FB96}` | INVERSE CHECKER BOARD FILL                                                               |
| 🮗    | `\u{1FB97}` | HEAVY HORIZONTAL FILL                                                                    |
| 🮘    | `\u{1FB98}` | UPPER LEFT TO LOWER RIGHT FILL                                                           |
| 🮙    | `\u{1FB99}` | UPPER RIGHT TO LOWER LEFT FILL                                                           |
| 🮚    | `\u{1FB9A}` | UPPER AND LOWER TRIANGULAR HALF BLOCK                                                    |
| 🮛    | `\u{1FB9B}` | LEFT AND RIGHT TRIANGULAR HALF BLOCK                                                     |
| 🮜    | `\u{1FB9C}` | UPPER LEFT TRIANGULAR MEDIUM SHADE                                                       |
| 🮝    | `\u{1FB9D}` | UPPER RIGHT TRIANGULAR MEDIUM SHADE                                                      |
| 🮞    | `\u{1FB9E}` | LOWER RIGHT TRIANGULAR MEDIUM SHADE                                                      |
| 🮟    | `\u{1FB9F}` | LOWER LEFT TRIANGULAR MEDIUM SHADE                                                       |
| 🮠    | `\u{1FBA0}` | BOX DRAWINGS LIGHT DIAGONAL UPPER CENTRE TO MIDDLE LEFT                                  |
| 🮡    | `\u{1FBA1}` | BOX DRAWINGS LIGHT DIAGONAL UPPER CENTRE TO MIDDLE RIGHT                                 |
| 🮢    | `\u{1FBA2}` | BOX DRAWINGS LIGHT DIAGONAL MIDDLE LEFT TO LOWER CENTRE                                  |
| 🮣    | `\u{1FBA3}` | BOX DRAWINGS LIGHT DIAGONAL MIDDLE RIGHT TO LOWER CENTRE                                 |
| 🮤    | `\u{1FBA4}` | BOX DRAWINGS LIGHT DIAGONAL UPPER CENTRE TO MIDDLE LEFT TO LOWER CENTRE                  |
| 🮥    | `\u{1FBA5}` | BOX DRAWINGS LIGHT DIAGONAL UPPER CENTRE TO MIDDLE RIGHT TO LOWER CENTRE                 |
| 🮦    | `\u{1FBA6}` | BOX DRAWINGS LIGHT DIAGONAL MIDDLE LEFT TO LOWER CENTRE TO MIDDLE RIGHT                  |
| 🮧    | `\u{1FBA7}` | BOX DRAWINGS LIGHT DIAGONAL MIDDLE LEFT TO UPPER CENTRE TO MIDDLE RIGHT                  |
| 🮨    | `\u{1FBA8}` | BOX DRAWINGS LIGHT DIAGONAL UPPER CENTRE TO MIDDLE LEFT AND MIDDLE RIGHT TO LOWER CENTRE |
| 🮩    | `\u{1FBA9}` | BOX DRAWINGS LIGHT DIAGONAL UPPER CENTRE TO MIDDLE RIGHT AND MIDDLE LEFT TO LOWER CENTRE |
| 🮪    | `\u{1FBAA}` | BOX DRAWINGS LIGHT DIAGONAL UPPER CENTRE TO MIDDLE RIGHT TO LOWER CENTRE TO MIDDLE LEFT  |
| 🮫    | `\u{1FBAB}` | BOX DRAWINGS LIGHT DIAGONAL UPPER CENTRE TO MIDDLE LEFT TO LOWER CENTRE TO MIDDLE RIGHT  |
| 🮬    | `\u{1FBAC}` | BOX DRAWINGS LIGHT DIAGONAL MIDDLE LEFT TO UPPER CENTRE TO MIDDLE RIGHT TO LOWER CENTRE  |
| 🮭    | `\u{1FBAD}` | BOX DRAWINGS LIGHT DIAGONAL MIDDLE RIGHT TO UPPER CENTRE TO MIDDLE LEFT TO LOWER CENTRE  |
| 🮮    | `\u{1FBAE}` | BOX DRAWINGS LIGHT DIAGONAL DIAMOND                                                      |
| 🮯    | `\u{1FBAF}` | BOX DRAWINGS LIGHT HORIZONTAL WITH VERTICAL STROKE                                       |
| 🮰    | `\u{1FBB0}` | ARROWHEAD-SHAPED POINTER                                                                 |
| 🮱    | `\u{1FBB1}` | INVERSE CHECK MARK                                                                       |
| 🮲    | `\u{1FBB2}` | LEFT HALF RUNNING MAN                                                                    |
| 🮳    | `\u{1FBB3}` | RIGHT HALF RUNNING MAN                                                                   |
| 🮴    | `\u{1FBB4}` | INVERSE DOWNWARDS ARROW WITH TIP LEFTWARDS                                               |
| 🮵    | `\u{1FBB5}` | LEFTWARDS ARROW AND UPPER AND LOWER ONE EIGHTH BLOCK                                     |
| 🮶    | `\u{1FBB6}` | RIGHTWARDS ARROW AND UPPER AND LOWER ONE EIGHTH BLOCK                                    |
| 🮷    | `\u{1FBB7}` | DOWNWARDS ARROW AND RIGHT ONE EIGHTH BLOCK                                               |
| 🮸    | `\u{1FBB8}` | UPWARDS ARROW AND RIGHT ONE EIGHTH BLOCK                                                 |
| 🮹    | `\u{1FBB9}` | LEFT HALF FOLDER                                                                         |
| 🮺    | `\u{1FBBA}` | RIGHT HALF FOLDER                                                                        |
| 🮻    | `\u{1FBBB}` | VOIDED GREEK CROSS                                                                       |
| 🮼    | `\u{1FBBC}` | RIGHT OPEN SQUARED DOT                                                                   |
| 🮽    | `\u{1FBBD}` | NEGATIVE DIAGONAL CROSS                                                                  |
| 🮾    | `\u{1FBBE}` | NEGATIVE DIAGONAL MIDDLE RIGHT TO LOWER CENTRE                                           |
| 🮿    | `\u{1FBBF}` | NEGATIVE DIAGONAL DIAMOND                                                                |
| 🯀    | `\u{1FBC0}` | WHITE HEAVY SALTIRE WITH ROUNDED CORNERS                                                 |
| 🯁    | `\u{1FBC1}` | LEFT THIRD WHITE RIGHT POINTING INDEX                                                    |
| 🯂    | `\u{1FBC2}` | MIDDLE THIRD WHITE RIGHT POINTING INDEX                                                  |
| 🯃    | `\u{1FBC3}` | RIGHT THIRD WHITE RIGHT POINTING INDEX                                                   |
| 🯄    | `\u{1FBC4}` | NEGATIVE SQUARED QUESTION MARK                                                           |
| 🯅    | `\u{1FBC5}` | STICK FIGURE                                                                             |
| 🯆    | `\u{1FBC6}` | STICK FIGURE WITH ARMS RAISED                                                            |
| 🯇    | `\u{1FBC7}` | STICK FIGURE LEANING LEFT                                                                |
| 🯈    | `\u{1FBC8}` | STICK FIGURE LEANING RIGHT                                                               |
| 🯉    | `\u{1FBC9}` | STICK FIGURE WITH DRESS                                                                  |
| 🯊    | `\u{1FBCA}` | WHITE UP-POINTING CHEVRON                                                                |
| 🯋    | `\u{1FBCB}` | WHITE CROSS MARK                                                                         |
| 🯌    | `\u{1FBCC}` | RAISED SMALL LEFT SQUARE BRACKET                                                         |
| 🯍    | `\u{1FBCD}` | BLACK SMALL UP-POINTING CHEVRON                                                          |
| 🯎    | `\u{1FBCE}` | LEFT TWO THIRDS BLOCK                                                                    |
| 🯏    | `\u{1FBCF}` | LEFT ONE THIRD BLOCK                                                                     |
| 🯐    | `\u{1FBD0}` | BOX DRAWINGS LIGHT DIAGONAL MIDDLE RIGHT TO LOWER LEFT                                   |
| 🯑    | `\u{1FBD1}` | BOX DRAWINGS LIGHT DIAGONAL UPPER RIGHT TO MIDDLE LEFT                                   |
| 🯒    | `\u{1FBD2}` | BOX DRAWINGS LIGHT DIAGONAL UPPER LEFT TO MIDDLE RIGHT                                   |
| 🯓    | `\u{1FBD3}` | BOX DRAWINGS LIGHT DIAGONAL MIDDLE LEFT TO LOWER RIGHT                                   |
| 🯔    | `\u{1FBD4}` | BOX DRAWINGS LIGHT DIAGONAL UPPER LEFT TO LOWER CENTRE                                   |
| 🯕    | `\u{1FBD5}` | BOX DRAWINGS LIGHT DIAGONAL UPPER CENTRE TO LOWER RIGHT                                  |
| 🯖    | `\u{1FBD6}` | BOX DRAWINGS LIGHT DIAGONAL UPPER RIGHT TO LOWER CENTRE                                  |
| 🯗    | `\u{1FBD7}` | BOX DRAWINGS LIGHT DIAGONAL UPPER CENTRE TO LOWER LEFT                                   |
| 🯘    | `\u{1FBD8}` | BOX DRAWINGS LIGHT DIAGONAL UPPER LEFT TO MIDDLE CENTRE TO UPPER RIGHT                   |
| 🯙    | `\u{1FBD9}` | BOX DRAWINGS LIGHT DIAGONAL UPPER RIGHT TO MIDDLE CENTRE TO LOWER RIGHT                  |
| 🯚    | `\u{1FBDA}` | BOX DRAWINGS LIGHT DIAGONAL LOWER LEFT TO MIDDLE CENTRE TO LOWER RIGHT                   |
| 🯛    | `\u{1FBDB}` | BOX DRAWINGS LIGHT DIAGONAL UPPER LEFT TO MIDDLE CENTRE TO LOWER LEFT                    |
| 🯜    | `\u{1FBDC}` | BOX DRAWINGS LIGHT DIAGONAL UPPER LEFT TO LOWER CENTRE TO UPPER RIGHT                    |
| 🯝    | `\u{1FBDD}` | BOX DRAWINGS LIGHT DIAGONAL UPPER RIGHT TO MIDDLE LEFT TO LOWER RIGHT                    |
| 🯞    | `\u{1FBDE}` | BOX DRAWINGS LIGHT DIAGONAL LOWER LEFT TO UPPER CENTRE TO LOWER RIGHT                    |
| 🯟    | `\u{1FBDF}` | BOX DRAWINGS LIGHT DIAGONAL UPPER LEFT TO MIDDLE RIGHT TO LOWER LEFT                     |
| 🯠    | `\u{1FBE0}` | TOP JUSTIFIED LOWER HALF WHITE CIRCLE                                                    |
| 🯡    | `\u{1FBE1}` | RIGHT JUSTIFIED LEFT HALF WHITE CIRCLE                                                   |
| 🯢    | `\u{1FBE2}` | BOTTOM JUSTIFIED UPPER HALF WHITE CIRCLE                                                 |
| 🯣    | `\u{1FBE3}` | LEFT JUSTIFIED RIGHT HALF WHITE CIRCLE                                                   |
| 🯤    | `\u{1FBE4}` | UPPER CENTRE ONE QUARTER BLOCK                                                           |
| 🯥    | `\u{1FBE5}` | LOWER CENTRE ONE QUARTER BLOCK                                                           |
| 🯦    | `\u{1FBE6}` | MIDDLE LEFT ONE QUARTER BLOCK                                                            |
| 🯧    | `\u{1FBE7}` | MIDDLE RIGHT ONE QUARTER BLOCK                                                           |
| 🯨    | `\u{1FBE8}` | TOP JUSTIFIED LOWER HALF BLACK CIRCLE                                                    |
| 🯩    | `\u{1FBE9}` | RIGHT JUSTIFIED LEFT HALF BLACK CIRCLE                                                   |
| 🯪    | `\u{1FBEA}` | BOTTOM JUSTIFIED UPPER HALF BLACK CIRCLE                                                 |
| 🯫    | `\u{1FBEB}` | LEFT JUSTIFIED RIGHT HALF BLACK CIRCLE                                                   |
| 🯬    | `\u{1FBEC}` | TOP RIGHT JUSTIFIED LOWER LEFT QUARTER BLACK CIRCLE                                      |
| 🯭    | `\u{1FBED}` | BOTTOM LEFT JUSTIFIED UPPER RIGHT QUARTER BLACK CIRCLE                                   |
| 🯮    | `\u{1FBEE}` | BOTTOM RIGHT JUSTIFIED UPPER LEFT QUARTER BLACK CIRCLE                                   |
| 🯯    | `\u{1FBEF}` | TOP LEFT JUSTIFIED LOWER RIGHT QUARTER BLACK CIRCLE                                      |
| 🯰    | `\u{1FBF0}` | SEGMENTED DIGIT ZERO                                                                     |
| 🯱    | `\u{1FBF1}` | SEGMENTED DIGIT ONE                                                                      |
| 🯲    | `\u{1FBF2}` | SEGMENTED DIGIT TWO                                                                      |
| 🯳    | `\u{1FBF3}` | SEGMENTED DIGIT THREE                                                                    |
| 🯴    | `\u{1FBF4}` | SEGMENTED DIGIT FOUR                                                                     |
| 🯵    | `\u{1FBF5}` | SEGMENTED DIGIT FIVE                                                                     |
| 🯶    | `\u{1FBF6}` | SEGMENTED DIGIT SIX                                                                      |
| 🯷    | `\u{1FBF7}` | SEGMENTED DIGIT SEVEN                                                                    |
| 🯸    | `\u{1FBF8}` | SEGMENTED DIGIT EIGHT                                                                    |
| 🯹    | `\u{1FBF9}` | SEGMENTED DIGIT NINE                                                                     |
| 🯺    | `\u{1FBFA}` |                                                                                          |
| 🯻    | `\u{1FBFB}` |                                                                                          |
| 🯼    | `\u{1FBFC}` |                                                                                          |
| 🯽    | `\u{1FBFD}` |                                                                                          |
| 🯾    | `\u{1FBFE}` |                                                                                          |
| 🯿    | `\u{1FBFF}` |                                                                                          |

## Latin-1 Supplement (96)

| Char | Escape     | Name                                       |
|------|------------|--------------------------------------------|
|      | `\u{00A0}` | NO-BREAK SPACE                             |
| ¡    | `\u{00A1}` | INVERTED EXCLAMATION MARK                  |
| ¢    | `\u{00A2}` | CENT SIGN                                  |
| £    | `\u{00A3}` | POUND SIGN                                 |
| ¤    | `\u{00A4}` | CURRENCY SIGN                              |
| ¥    | `\u{00A5}` | YEN SIGN                                   |
| ¦    | `\u{00A6}` | BROKEN BAR                                 |
| §    | `\u{00A7}` | SECTION SIGN                               |
| ¨    | `\u{00A8}` | DIAERESIS                                  |
| ©    | `\u{00A9}` | COPYRIGHT SIGN                             |
| ª    | `\u{00AA}` | FEMININE ORDINAL INDICATOR                 |
| «    | `\u{00AB}` | LEFT-POINTING DOUBLE ANGLE QUOTATION MARK  |
| ¬    | `\u{00AC}` | NOT SIGN                                   |
| ­     | `\u{00AD}` | SOFT HYPHEN                                |
| ®    | `\u{00AE}` | REGISTERED SIGN                            |
| ¯    | `\u{00AF}` | MACRON                                     |
| °    | `\u{00B0}` | DEGREE SIGN                                |
| ±    | `\u{00B1}` | PLUS-MINUS SIGN                            |
| ²    | `\u{00B2}` | SUPERSCRIPT TWO                            |
| ³    | `\u{00B3}` | SUPERSCRIPT THREE                          |
| ´    | `\u{00B4}` | ACUTE ACCENT                               |
| µ    | `\u{00B5}` | MICRO SIGN                                 |
| ¶    | `\u{00B6}` | PILCROW SIGN                               |
| ·    | `\u{00B7}` | MIDDLE DOT                                 |
| ¸    | `\u{00B8}` | CEDILLA                                    |
| ¹    | `\u{00B9}` | SUPERSCRIPT ONE                            |
| º    | `\u{00BA}` | MASCULINE ORDINAL INDICATOR                |
| »    | `\u{00BB}` | RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK |
| ¼    | `\u{00BC}` | VULGAR FRACTION ONE QUARTER                |
| ½    | `\u{00BD}` | VULGAR FRACTION ONE HALF                   |
| ¾    | `\u{00BE}` | VULGAR FRACTION THREE QUARTERS             |
| ¿    | `\u{00BF}` | INVERTED QUESTION MARK                     |
| À    | `\u{00C0}` | LATIN CAPITAL LETTER A WITH GRAVE          |
| Á    | `\u{00C1}` | LATIN CAPITAL LETTER A WITH ACUTE          |
| Â    | `\u{00C2}` | LATIN CAPITAL LETTER A WITH CIRCUMFLEX     |
| Ã    | `\u{00C3}` | LATIN CAPITAL LETTER A WITH TILDE          |
| Ä    | `\u{00C4}` | LATIN CAPITAL LETTER A WITH DIAERESIS      |
| Å    | `\u{00C5}` | LATIN CAPITAL LETTER A WITH RING ABOVE     |
| Æ    | `\u{00C6}` | LATIN CAPITAL LETTER AE                    |
| Ç    | `\u{00C7}` | LATIN CAPITAL LETTER C WITH CEDILLA        |
| È    | `\u{00C8}` | LATIN CAPITAL LETTER E WITH GRAVE          |
| É    | `\u{00C9}` | LATIN CAPITAL LETTER E WITH ACUTE          |
| Ê    | `\u{00CA}` | LATIN CAPITAL LETTER E WITH CIRCUMFLEX     |
| Ë    | `\u{00CB}` | LATIN CAPITAL LETTER E WITH DIAERESIS      |
| Ì    | `\u{00CC}` | LATIN CAPITAL LETTER I WITH GRAVE          |
| Í    | `\u{00CD}` | LATIN CAPITAL LETTER I WITH ACUTE          |
| Î    | `\u{00CE}` | LATIN CAPITAL LETTER I WITH CIRCUMFLEX     |
| Ï    | `\u{00CF}` | LATIN CAPITAL LETTER I WITH DIAERESIS      |
| Ð    | `\u{00D0}` | LATIN CAPITAL LETTER ETH                   |
| Ñ    | `\u{00D1}` | LATIN CAPITAL LETTER N WITH TILDE          |
| Ò    | `\u{00D2}` | LATIN CAPITAL LETTER O WITH GRAVE          |
| Ó    | `\u{00D3}` | LATIN CAPITAL LETTER O WITH ACUTE          |
| Ô    | `\u{00D4}` | LATIN CAPITAL LETTER O WITH CIRCUMFLEX     |
| Õ    | `\u{00D5}` | LATIN CAPITAL LETTER O WITH TILDE          |
| Ö    | `\u{00D6}` | LATIN CAPITAL LETTER O WITH DIAERESIS      |
| ×    | `\u{00D7}` | MULTIPLICATION SIGN                        |
| Ø    | `\u{00D8}` | LATIN CAPITAL LETTER O WITH STROKE         |
| Ù    | `\u{00D9}` | LATIN CAPITAL LETTER U WITH GRAVE          |
| Ú    | `\u{00DA}` | LATIN CAPITAL LETTER U WITH ACUTE          |
| Û    | `\u{00DB}` | LATIN CAPITAL LETTER U WITH CIRCUMFLEX     |
| Ü    | `\u{00DC}` | LATIN CAPITAL LETTER U WITH DIAERESIS      |
| Ý    | `\u{00DD}` | LATIN CAPITAL LETTER Y WITH ACUTE          |
| Þ    | `\u{00DE}` | LATIN CAPITAL LETTER THORN                 |
| ß    | `\u{00DF}` | LATIN SMALL LETTER SHARP S                 |
| à    | `\u{00E0}` | LATIN SMALL LETTER A WITH GRAVE            |
| á    | `\u{00E1}` | LATIN SMALL LETTER A WITH ACUTE            |
| â    | `\u{00E2}` | LATIN SMALL LETTER A WITH CIRCUMFLEX       |
| ã    | `\u{00E3}` | LATIN SMALL LETTER A WITH TILDE            |
| ä    | `\u{00E4}` | LATIN SMALL LETTER A WITH DIAERESIS        |
| å    | `\u{00E5}` | LATIN SMALL LETTER A WITH RING ABOVE       |
| æ    | `\u{00E6}` | LATIN SMALL LETTER AE                      |
| ç    | `\u{00E7}` | LATIN SMALL LETTER C WITH CEDILLA          |
| è    | `\u{00E8}` | LATIN SMALL LETTER E WITH GRAVE            |
| é    | `\u{00E9}` | LATIN SMALL LETTER E WITH ACUTE            |
| ê    | `\u{00EA}` | LATIN SMALL LETTER E WITH CIRCUMFLEX       |
| ë    | `\u{00EB}` | LATIN SMALL LETTER E WITH DIAERESIS        |
| ì    | `\u{00EC}` | LATIN SMALL LETTER I WITH GRAVE            |
| í    | `\u{00ED}` | LATIN SMALL LETTER I WITH ACUTE            |
| î    | `\u{00EE}` | LATIN SMALL LETTER I WITH CIRCUMFLEX       |
| ï    | `\u{00EF}` | LATIN SMALL LETTER I WITH DIAERESIS        |
| ð    | `\u{00F0}` | LATIN SMALL LETTER ETH                     |
| ñ    | `\u{00F1}` | LATIN SMALL LETTER N WITH TILDE            |
| ò    | `\u{00F2}` | LATIN SMALL LETTER O WITH GRAVE            |
| ó    | `\u{00F3}` | LATIN SMALL LETTER O WITH ACUTE            |
| ô    | `\u{00F4}` | LATIN SMALL LETTER O WITH CIRCUMFLEX       |
| õ    | `\u{00F5}` | LATIN SMALL LETTER O WITH TILDE            |
| ö    | `\u{00F6}` | LATIN SMALL LETTER O WITH DIAERESIS        |
| ÷    | `\u{00F7}` | DIVISION SIGN                              |
| ø    | `\u{00F8}` | LATIN SMALL LETTER O WITH STROKE           |
| ù    | `\u{00F9}` | LATIN SMALL LETTER U WITH GRAVE            |
| ú    | `\u{00FA}` | LATIN SMALL LETTER U WITH ACUTE            |
| û    | `\u{00FB}` | LATIN SMALL LETTER U WITH CIRCUMFLEX       |
| ü    | `\u{00FC}` | LATIN SMALL LETTER U WITH DIAERESIS        |
| ý    | `\u{00FD}` | LATIN SMALL LETTER Y WITH ACUTE            |
| þ    | `\u{00FE}` | LATIN SMALL LETTER THORN                   |
| ÿ    | `\u{00FF}` | LATIN SMALL LETTER Y WITH DIAERESIS        |

## CJK Compatibility (111)

| Char | Escape     | Name                      |
|------|------------|---------------------------|
| ㍱   | `\u{3371}` | SQUARE HPA                |
| ㍲   | `\u{3372}` | SQUARE DA                 |
| ㍳   | `\u{3373}` | SQUARE AU                 |
| ㍴   | `\u{3374}` | SQUARE BAR                |
| ㍵   | `\u{3375}` | SQUARE OV                 |
| ㍶   | `\u{3376}` | SQUARE PC                 |
| ㍷   | `\u{3377}` | SQUARE DM                 |
| ㍸   | `\u{3378}` | SQUARE DM SQUARED         |
| ㍹   | `\u{3379}` | SQUARE DM CUBED           |
| ㍺   | `\u{337A}` | SQUARE IU                 |
| ㍻   | `\u{337B}` | SQUARE ERA NAME HEISEI    |
| ㍼   | `\u{337C}` | SQUARE ERA NAME SYOUWA    |
| ㍽   | `\u{337D}` | SQUARE ERA NAME TAISYOU   |
| ㍾   | `\u{337E}` | SQUARE ERA NAME MEIZI     |
| ㍿   | `\u{337F}` | SQUARE CORPORATION        |
| ㎀   | `\u{3380}` | SQUARE PA AMPS            |
| ㎁   | `\u{3381}` | SQUARE NA                 |
| ㎂   | `\u{3382}` | SQUARE MU A               |
| ㎃   | `\u{3383}` | SQUARE MA                 |
| ㎄   | `\u{3384}` | SQUARE KA                 |
| ㎅   | `\u{3385}` | SQUARE KB                 |
| ㎆   | `\u{3386}` | SQUARE MB                 |
| ㎇   | `\u{3387}` | SQUARE GB                 |
| ㎈   | `\u{3388}` | SQUARE CAL                |
| ㎉   | `\u{3389}` | SQUARE KCAL               |
| ㎊   | `\u{338A}` | SQUARE PF                 |
| ㎋   | `\u{338B}` | SQUARE NF                 |
| ㎌   | `\u{338C}` | SQUARE MU F               |
| ㎍   | `\u{338D}` | SQUARE MU G               |
| ㎎   | `\u{338E}` | SQUARE MG                 |
| ㎏   | `\u{338F}` | SQUARE KG                 |
| ㎐   | `\u{3390}` | SQUARE HZ                 |
| ㎑   | `\u{3391}` | SQUARE KHZ                |
| ㎒   | `\u{3392}` | SQUARE MHZ                |
| ㎓   | `\u{3393}` | SQUARE GHZ                |
| ㎔   | `\u{3394}` | SQUARE THZ                |
| ㎕   | `\u{3395}` | SQUARE MU L               |
| ㎖   | `\u{3396}` | SQUARE ML                 |
| ㎗   | `\u{3397}` | SQUARE DL                 |
| ㎘   | `\u{3398}` | SQUARE KL                 |
| ㎙   | `\u{3399}` | SQUARE FM                 |
| ㎚   | `\u{339A}` | SQUARE NM                 |
| ㎛   | `\u{339B}` | SQUARE MU M               |
| ㎜   | `\u{339C}` | SQUARE MM                 |
| ㎝   | `\u{339D}` | SQUARE CM                 |
| ㎞   | `\u{339E}` | SQUARE KM                 |
| ㎟   | `\u{339F}` | SQUARE MM SQUARED         |
| ㎠   | `\u{33A0}` | SQUARE CM SQUARED         |
| ㎡   | `\u{33A1}` | SQUARE M SQUARED          |
| ㎢   | `\u{33A2}` | SQUARE KM SQUARED         |
| ㎣   | `\u{33A3}` | SQUARE MM CUBED           |
| ㎤   | `\u{33A4}` | SQUARE CM CUBED           |
| ㎥   | `\u{33A5}` | SQUARE M CUBED            |
| ㎦   | `\u{33A6}` | SQUARE KM CUBED           |
| ㎧   | `\u{33A7}` | SQUARE M OVER S           |
| ㎨   | `\u{33A8}` | SQUARE M OVER S SQUARED   |
| ㎩   | `\u{33A9}` | SQUARE PA                 |
| ㎪   | `\u{33AA}` | SQUARE KPA                |
| ㎫   | `\u{33AB}` | SQUARE MPA                |
| ㎬   | `\u{33AC}` | SQUARE GPA                |
| ㎭   | `\u{33AD}` | SQUARE RAD                |
| ㎮   | `\u{33AE}` | SQUARE RAD OVER S         |
| ㎯   | `\u{33AF}` | SQUARE RAD OVER S SQUARED |
| ㎰   | `\u{33B0}` | SQUARE PS                 |
| ㎱   | `\u{33B1}` | SQUARE NS                 |
| ㎲   | `\u{33B2}` | SQUARE MU S               |
| ㎳   | `\u{33B3}` | SQUARE MS                 |
| ㎴   | `\u{33B4}` | SQUARE PV                 |
| ㎵   | `\u{33B5}` | SQUARE NV                 |
| ㎶   | `\u{33B6}` | SQUARE MU V               |
| ㎷   | `\u{33B7}` | SQUARE MV                 |
| ㎸   | `\u{33B8}` | SQUARE KV                 |
| ㎹   | `\u{33B9}` | SQUARE MV MEGA            |
| ㎺   | `\u{33BA}` | SQUARE PW                 |
| ㎻   | `\u{33BB}` | SQUARE NW                 |
| ㎼   | `\u{33BC}` | SQUARE MU W               |
| ㎽   | `\u{33BD}` | SQUARE MW                 |
| ㎾   | `\u{33BE}` | SQUARE KW                 |
| ㎿   | `\u{33BF}` | SQUARE MW MEGA            |
| ㏀   | `\u{33C0}` | SQUARE K OHM              |
| ㏁   | `\u{33C1}` | SQUARE M OHM              |
| ㏂   | `\u{33C2}` | SQUARE AM                 |
| ㏃   | `\u{33C3}` | SQUARE BQ                 |
| ㏄   | `\u{33C4}` | SQUARE CC                 |
| ㏅   | `\u{33C5}` | SQUARE CD                 |
| ㏆   | `\u{33C6}` | SQUARE C OVER KG          |
| ㏇   | `\u{33C7}` | SQUARE CO                 |
| ㏈   | `\u{33C8}` | SQUARE DB                 |
| ㏉   | `\u{33C9}` | SQUARE GY                 |
| ㏊   | `\u{33CA}` | SQUARE HA                 |
| ㏋   | `\u{33CB}` | SQUARE HP                 |
| ㏌   | `\u{33CC}` | SQUARE IN                 |
| ㏍   | `\u{33CD}` | SQUARE KK                 |
| ㏎   | `\u{33CE}` | SQUARE KM CAPITAL         |
| ㏏   | `\u{33CF}` | SQUARE KT                 |
| ㏐   | `\u{33D0}` | SQUARE LM                 |
| ㏑   | `\u{33D1}` | SQUARE LN                 |
| ㏒   | `\u{33D2}` | SQUARE LOG                |
| ㏓   | `\u{33D3}` | SQUARE LX                 |
| ㏔   | `\u{33D4}` | SQUARE MB SMALL           |
| ㏕   | `\u{33D5}` | SQUARE MIL                |
| ㏖   | `\u{33D6}` | SQUARE MOL                |
| ㏗   | `\u{33D7}` | SQUARE PH                 |
| ㏘   | `\u{33D8}` | SQUARE PM                 |
| ㏙   | `\u{33D9}` | SQUARE PPM                |
| ㏚   | `\u{33DA}` | SQUARE PR                 |
| ㏛   | `\u{33DB}` | SQUARE SR                 |
| ㏜   | `\u{33DC}` | SQUARE SV                 |
| ㏝   | `\u{33DD}` | SQUARE WB                 |
| ㏞   | `\u{33DE}` | SQUARE V OVER M           |
| ㏟   | `\u{33DF}` | SQUARE A OVER M           |

