##!/usr/bin/env python3

import re
import os
import json
from subprocess import Popen, PIPE



# get tigershark executable
target="debug"
# target="release"

# for now the Rust executable path is hard-coded...
pwd = "/home/bunker/projects/tigershark/target/"
command = pwd+target+"/tigershark"



# return tuple with (ErrorCode,output)
def db_insert(myjson):

    try:
        process = Popen([command,"-i",json.dumps(myjson)], stdout=PIPE)
        (output, err) = process.communicate()
        exit_code = process.wait()
        output = output.decode('utf-8')
        if exit_code == 0:
            return (0,output)
        else:
            return (1,output)

    except:
        return (1,"Python Popen failed")

