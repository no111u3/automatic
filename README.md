# Automatic
`automatic` runs command through user defined scripts

Commands collect to run items in run list.

For use this program you can write some script in yaml:

```yaml
---
Promiscuous:
  items:
    - name: "true"
      args: []
    - name: "true"
      args: []
    - name: "true"
      args: []

```

Than you can run throw command line application:

`automatic -r <your_script_name>`

`Promiscuous` is promiscuous type of run, no output, no collect any errors, but
if run anything program was fail - you get an error of run, such as:
`Failed to run script with error: fail to read script with error: Is a directory (os error 21)`
