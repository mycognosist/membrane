# membrane

Selectively-permeable membrane. Filter `stdin` and print matches to `stdout`.

_Designed with [enie](https://github.com/mycognosist/enie) in mind._

### Description

`membrane` reads from `stdin` and filters the results according to the selectors passed in as arguments. Each incoming line from `stdin` is checked to see if it starts with any of the given selectors. If a match is found, the source line is printed to `stdout`.

### Usage

`enie <iface> | membrane <selector(s)>`

Example:

`enie wlan0 | membrane 2 3`

Output:

`3_wlan0`  
`2_wlan0_10.10.10.7`

Filtered items (not output by `membrane`):

`0_wlan0_UP_NOT RUNNING`  
`0_wlan0_UP_RUNNING`
