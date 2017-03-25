[![Build Status](https://travis-ci.org/boggad/regex-tester.svg?branch=master)](https://travis-ci.org/boggad/regex-tester)

# regex-tester
The regex-tester is a command-line utility that helps to check and analyze regular expressions.

## Get started
Regex-tester will greet you with command prompt:

```
============================================================================================
=== Welcome to regex-tester! Start by entering a command. Type 'help' to show help page. ===
============================================================================================
#> █
```

Start by setting regular expression with `re` command. Suppose we want to check date format regex:
```
#> re (\d{2})(/|\.)(\d{2})(/|\.)(\d{4})
Regex was set: (\d{2})(/|\.)(\d{2})(/|\.)(\d{4})

#> 
```

Now let's test if some date strings match the expression with `test` command:

```
#> test 02.02.2017
>>> Match #1, capture groups: 
>>> <0>: 02.02.2017
>>> <1>: 02
>>> <2>: .
>>> <3>: 02
>>> <4>: .
>>> <5>: 2017

#> test 20.20.2016 20/20/2017
>>> Match #1, capture groups: 
>>> <0>: 20.20.2016
>>> <1>: 20
>>> <2>: .
>>> <3>: 20
>>> <4>: .
>>> <5>: 2016
===========
>>> Match #2, capture groups: 
>>> <0>: 20/20/2017
>>> <1>: 20
>>> <2>: /
>>> <3>: 20
>>> <4>: /
>>> <5>: 2017

#>
```

You can set new regular expression with `re` command. Also, you can navigate through history of commands with up/down arrow keys.

To exit, press `<Ctrl-D>` or enter `exit` command:
