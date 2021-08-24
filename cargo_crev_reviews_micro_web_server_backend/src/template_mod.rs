// template_mood

pub fn template_html() -> &'static str {
    r##"
<!DOCTYPE html>
<html lang="en">

<head>
    <meta http-equiv="Content-type" content="text/html; charset=utf-8" />
    <title>cargo_crev_review</title>
    <meta name="Description" content="cargo_crev_review" />
    <link rel="stylesheet" href="css/cargo_crev_reviews.css" />
    <meta name="viewport" content="width = device-width,initial-scale = 1.0" />
    <link rel="shortcut icon" type="image/x-icon" href="favicon.png" />
</head>

<body>
    Hello world!
</body>

</html>
"##
}

pub fn css() -> &'static str {
    r##"
/* css variables */
    
:root {
    /* color palette */
    /* use of variables: var(--color_tooltip_1); */
    /* background color */
    --b_color_body: #000000;
    --b_color_container: #111111;
    --b_color_grid_header: #1B1B1B;
    --b_color_code: #000000;
    --b_color_cell_rating: #000000;
    --b_color_button: dodgerblue;
    /* front color */
    --f_color_body: #9DA5B4;
    --f_color_code: #78C379;
    --f_color_link: #ffffff;
    --f_color_05: #FF9900;
    --f_color_06: dark-white;
    --f_color_07: black;
    /* border color*/
    --brd_color_grid: #313131;
    --brd_color_container: #313131;
    /* color */
    --color_r_strong: Chartreuse;
    --color_r_positive: Green;
    --color_r_medium: orange;
    --color_r_neutral: #9DA5B4;
    --color_r_negative: red;
    --color_r_none: #9DA5B4;
    --color_tooltip_1: #000;
    --color_tooltip_2: hsla(0, 0%, 20%, 0.9);
    --color_tooltip_3: #fff;
}
/*region: media dependent on screen size */
/* less then 590px*/

@media (max-width: 590px) {
    .media_header_grid_01 {
        display: grid;
        grid-template-columns: 1fr;
    }
    .media_header_grid_02 {
        display: grid;
        grid-template-columns: 1fr;
    }
    .media_right {
        text-align: left;
    }
    .media_portrait_visible {
        visibility: visible;
    }
}
/* larger then 590px */

@media (min-width: 590px) {
    .media_header_grid_01 {
        display: grid;
        grid-template-columns: 1fr 3fr;
    }
    .media_header_grid_02 {
        display: grid;
        grid-template-columns: 4fr 9fr 3fr;
    }
    .media_right {
        text-align: right;
    }
    .media_portrait_visible {
        visibility: hidden;
    }
}
/* region: basics */

html {
    font-family: sans-serif;
    max-width: 1200px;
    min-width: 300px;
    width: 100%;
    /*margin auto means centered horizontally*/
    margin: auto;
    padding-right: 0px;
    overflow-y: auto;
    overflow-x: hidden;
    word-wrap: break-word;
    overflow-wrap: break-word;
    box-sizing: border-box;
    background-color: var(--b_color_body);
    line-height: 1.5;
    color: var(--f_color_body);
    /*This is the base font-size. All other font-size 
use rem units that are
relative to this font-size.*/
    /*width greater than 600 px*/
    font-size: 34px;
    -webkit-font-smoothing: antialiased;
    text-shadow: 1px 1px 1px rgba(0, 0, 0, 0.004);
}

body {
    max-width: 1200px;
    margin: 0;
    padding: 0;
    font-size: 60%;
    line-height: 1.5;
    background-color: var(--b_color_body);
    color: var(--f_color_body);
}
/*CSS Reset*/

div,
span,
applet,
object,
iframe,
h1,
h2,
h3,
h4,
h5,
h6,
p,
blockquote,
pre,
a,
abbr,
acronym,
address,
big,
cite,
code,
del,
dfn,
em,
font,
img,
ins,
kbd,
q,
s,
samp,
small,
strike,
strong,
sub,
sup,
tt,
var,
b,
u,
i,
center,
dl,
dt,
dd,
ol,
ul,
li,
fieldset,
form,
label,
legend,
table,
caption,
tbody,
tfoot,
thead,
tr,
th,
td {
    margin: 0;
    padding: 0;
    border: 0;
    border-style: none;
    outline: 0;
    font-family: 'Roboto', sans-serif;
    vertical-align: baseline;
    background: transparent;
}

/* no color */
a:link,
a:visited,
a:hover,
a:active {
    color: inherit;
    text-decoration: none;
}

h1,
h2,
h3,
h4 {
    margin-bottom: 16px;
}

p {
    line-height: 1.5;
}

pre {
    white-space: pre-wrap;
}

code {
    padding: .1em .4em;
    margin: 0;
    font-size: 85%;
    background-color: var(--b_color_code);
    color: var(--f_color_code);
    border-radius: 3px;
    font-family: Consolas, Liberation Mono, Courier, monospace;
}

input[type=text] {
    background-color: var(--b_color_code);
    color: var(--f_color_code);
    width: 600px;
    border: 1px;
    border-radius: 3px;
    padding: 2px;
    font-size: 100%;
    font-family: Consolas, Liberation Mono, Courier, monospace;
}

input.read_only {
    background-color: var(--f_color_link);
    color: var(--f_color_body);
}

textarea {
    background-color: var(--b_color_code);
    color: var(--f_color_code);
    width: 900px;
    height: 120px;
    border: 1px;
    border-radius: 3px;
    padding: 18px;
    font-size: 100%;
    font-family: Consolas, Liberation Mono, Courier, monospace;
}

ul {
    margin-left: 0; 
    padding-left: 40px;
    list-style-position: inside;
}

/* endregion: basics */
/* region: css classes */
/* When concatenating names that makes it hard to refactor later.
example if both `grid` and `grid_cell` start with the same word `grid`.
So I mandatory add a number. 
But to make them different to the searcher, 
the first one have underscore+number:               container0_content_grid_0
and the second one only number without underscore:  grid0_c
A small difference to help search and replace refactoring.
*/

.container0_content_grid_0 {
    width: 100%;
    display: grid;
    border-radius: 5px;
    border: 0.2px solid var(--brd_color_grid);
}

.grid0_h_c {
    /* grid0 header cell */
    background-color: var(--b_color_grid_header);
    border: 0.2px solid var(--brd_color_grid);
    text-align: center;
}

.grid0_c {
    /* grid0 cell */
    border: 0.2px solid var(--brd_color_grid);
    text-align: center;
}

.grid0_c_r {
    /* grid0 cell rating*/
    background-color: var(--b_color_cell_rating);
}
.container_0 {
    display: grid;
    background-color: var(--b_color_container);
    border: 2px solid var(--brd_color_grid);
    border-radius: 5px;
    margin: 2%;
}

.container0_content_not_grid {
    padding: 20px;
}

.review_header_0 {
    display: grid;
    justify-content: space-around;
    background-color: var(--b_color_grid_header);
    padding: 10px;
}

.review_header0_cell {
    padding: 0px;
    /*border: 1px solid var(--brd_color_grid);*/
    text-align: center;
}

.review_info_header {
    display: flex;
    justify-content: space-around;
    background-color: var(--b_color_grid_header);
    padding: 10px;
}

.review_comment {
    padding: 20px;
    padding-top: 20px;
    font-family: sans-serif;
    font-size: 20px;
    word-wrap: break-word;
    overflow-wrap: break-word;
    white-space: pre-line;
    overflow: hidden;
}

.h3y {
    /*h3 yellow ?/
/*The .class_name is repeated in css to take different properties.*/
    background: transparent;
}

.h2u {
    /*h2 underline */
    /*The .class_name is repeated in css to take different properties.*/
    background: transparent;
    margin-top: 20px;
}
/* endregion: css classes */
/* region: colors and attributes */

.break-all {
    word-break: break-all;
    word-wrap: break-word;
    overflow-wrap: break-word;
}

.word-wrap {
    word-wrap: break-word;
    overflow-wrap: break-all;
}

.bold {
    font-weight: bold;
}

/* links are mostly white */
.c_white,
.c_link_1,
a:link.c_link_1,
a:visited.c_link_1,
a:hover.c_link_1,
a:active.c_link_1
{
    color: var(--f_color_link);
}

/* dark_white */
.h2u {
    color: var(--f_color_06);
}

.c_black {
    color: var(--f_color_07);
}

/* greener */
.c_strong,
.c_high,
.c_low_severity {
    color: var(--color_r_strong);
}

/* green */
.c_positive {
    color: var(--color_r_positive);
}

/* yellow */
.c_yellow,
.c_alternative,
.c_medium_severity,
.h3y,
.c_link_2,
a:link.c_link_2,
a:visited.c_link_2,
a:hover.c_link_2,
a:active.c_link_2
{
    color: var(--f_color_05);
}

/* orange */
.c_medium,
.c_neutral,
.c_issue {
    color: var(--color_r_medium);
}

/* red */
.c_alert,
.c_negative,
.c_low,
.c_high_severity,
.c_advisory {
    color: var(--color_r_negative);
}

/* none rating */
.c_none
{
    color: var(--color_r_none);
}

/* neutral rating */
.c_neutral
{
    color: var(--color_r_neutral);
}

.middle {
    display: grid;
    align-items: center;
}

.top {
    display: grid;
    align-items: top;
}

.center {
    display: block;
    margin-left: auto;
    margin-right: auto;
}

.right {
    text-align: right;
}

.under_line,
.h2u {
    border-bottom: 1px solid var(--brd_color_container);
}

.big {
    font-size: 140%;
}

.small {
    font-size: 80%;
}
/* endregion: colors and attributes */

.button {
    background: var(--b_color_button);
    border-radius: 6px;
    font-size: 120%;
}
/*** Tooltip Styles */
/* Add this attribute to the element that needs a tooltip */

[data-tooltip] {
    position: relative;
    z-index: 2;
    cursor: pointer;
}
/* Hide the tooltip content by default */

[data-tooltip]:before,
[data-tooltip]:after {
    visibility: hidden;
    -ms-filter: "progid:DXImageTransform.Microsoft.Alpha(Opacity=0)";
    filter: progid: DXImageTransform.Microsoft.Alpha(Opacity=0);
    opacity: 0;
    pointer-events: none;
}
/* Position tooltip above the element */

[data-tooltip]:before {
    position: absolute;
    bottom: 150%;
    left: 50%;
    margin-bottom: 5px;
    margin-left: -80px;
    padding: 7px;
    width: 160px;
    -webkit-border-radius: 3px;
    -moz-border-radius: 3px;
    border-radius: 3px;
    background-color: var(--color_tooltip_1);
    background-color: var(--color_tooltip_2);
    color: var(--color_tooltip_3);
    content: attr(data-tooltip);
    text-align: center;
    font-size: 14px;
    line-height: 1.2;
}
/* Triangle hack to make tooltip look like a speech bubble */

[data-tooltip]:after {
    position: absolute;
    bottom: 150%;
    left: 50%;
    margin-left: -5px;
    width: 0;
    border-top: 5px solid var(--color_tooltip_1);
    border-top: 5px solid var(--color_tooltip_2);
    border-right: 5px solid transparent;
    border-left: 5px solid transparent;
    content: " ";
    font-size: 0;
    line-height: 0;
}
/* Show tooltip content on hover */

[data-tooltip]:hover:before,
[data-tooltip]:hover:after {
    visibility: visible;
    -ms-filter: "progid:DXImageTransform.Microsoft.Alpha(Opacity=100)";
    filter: progid: DXImageTransform.Microsoft.Alpha(Opacity=100);
    opacity: 1;
}
/*special instructions*/

.blink {
    animation: blinker 1s linear infinite;
}

@keyframes blinker {
    0% {
        opacity: 1.0;
    }
    50% {
        opacity: 1.0;
    }
    100% {
        opacity: 0.1;
    }
}
    "##
}

// encoded in base64
pub fn icon_original_png() -> &'static str {
    r##"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
AABJRU5ErkJggg=="##
}
