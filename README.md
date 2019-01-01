[![Build Status](https://travis-ci.org/KuldeepSinh/lc3_vm.svg?branch=master)](https://travis-ci.org/KuldeepSinh/lc3_vm)

# lc3_vm
LC-3 (Little Computer 3) VM implemented in Rust. Ideally, it will run any LC-3 assembly program. 

## Motivation : 
* [Write your Own Virtual Machine](https://justinmeiners.github.io/lc3-vm/index.html#1:12)
* [Wiki : LC-3](https://en.wikipedia.org/wiki/LC-3)

### Example#1 : Play 2048!

```bash
$ cargo run ./resources/2048.obj
```

### Result

```
+--------------------------+
|                          |
|         2                |
|                          |
|                          |
|                          |
|   2                      |
|                          |
|                          |
|                          |
+--------------------------+

```

### Example#2 : Play Rogue!

```bash
$ cargo run ./resources/rogue.obj
```

### Result

```
##################  ############
###################     ########
#######################        #
########################  #  #  
###############################D
################################
################################
@ ##############################
#  #############################
##    ##########################
#####  #########################
######  ########################
#######   ######################
#########    ###################
############  ##  ##############
#############      #############

```
