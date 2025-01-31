
<a name="v0.8.0"></a>
### v0.8.0 - 2019-11-24
- Templates allow going further in separating form from content

<a name="v0.7.6"></a>
### v0.7.6 - 2019-11-15
- fix skin's background not applied on empty lines at end of text_view
- use version minimad 0.4.3 to fix case of code not detected when following italic without space in between

<a name="v0.7.5"></a>
### v0.7.5 - 2019-11-13
- fix skin's background not applied on empty lines at end of text_view

<a name="v0.7.4"></a>
### v0.7.4 - 2019-11-11
- introduce inline templates, and especially the `mad_print_inline!` and `mad_write_inline!` macros
- add functions to shrink or extend a composite to a given width, using internal elision if possible

<a name="v0.7.3"></a>
### v0.7.3 - 2019-11-08
- add easy alternate to `skin.print_text` handling IO error

<a name="v0.7.2"></a>
### v0.7.2 - 2019-11-04
- incorporate crossterm 0.13.2 which fixes a regression on input reader

<a name="v0.7.1"></a>
### v0.7.1 - 2019-11-03
- compatibility with crossterm 0.13
- mouse support in stderr

<a name="v0.7.0"></a>
### v0.7.0 - 2019-09-22
- Displaying can be done on stderr or stdout, or in a subshell

<a name="v0.6.6"></a>
### v0.6.6 - 2019-10-05
- provide a default terminal width when the real one can't be measured

<a name="v0.6.5"></a>
### v0.6.5 - 2019-08-31
- list view: autoscroll on selection change
- list view: select_first_line & select_last_line

<a name="v0.6.4"></a>
### v0.6.4 - 2019-08-02
- add ProgressBar

<a name="v0.6.3"></a>
### v0.6.3 - 2019-08-01
- improvements of ListView

<a name="v0.6.2"></a>
### v0.6.2 - 2019-07-31
- fix build inconsistencies due to lack of precise sub crate versionning in crossterm

<a name="v0.6.1"></a>
### v0.6.1 - 2019-07-29
- add modifiable style for the input_field

<a name="v0.6.0"></a>
### v0.6.0 - 2019-07-28
Some tools that were parts of several Termimad based applications are now shared here:
- an event source emmiting events on a crossbeam channel
- an input field
- a list view with auto-resized columns

<a name="v0.5.1"></a>
### v0.5.1 - 2019-07-21
- a few utilies exported for apps with specific usages (compute_scrollbar, spacing.print_str, etc.)

<a name="v0.5.0"></a>
### v0.5.0 - 2019-07-09
- different styles for inline_code and code_block
- rgb now longer a macro but a free function
- two more color functions: ansi and gray
- clean and complete documentation

<a name="v0.4.0"></a>
### v0.4.0 - 2019-07-02
- support for horizontal rule (line made of dashes)
- support for quote (line starting with '>')
- support for bullet style customization (including colors)
- better wrapping, less frequently breaks words
- Skin API *breaking changes* to allow for more customization

