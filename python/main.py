import re
import os
import json
from subprocess import Popen, PIPE

# get tigershark executable
target="debug"
# target="release"

# until I figure out a proper fix...
# pwd is different inside/outside of Neovim when using Git
pwd = re.sub("/python","",os.getcwd())
command = pwd+"/target/"+target+"/tigershark"
print(">>" , command)

# create some json for test
myjson = {"name":"bob","some random stuff":433.34}

try:
    process = Popen([command,"-i",json.dumps(myjson)], stdout=PIPE)
    (output, err) = process.communicate()
    exit_code = process.wait()
    output = output.decode('utf-8')
    print("exit code: ",exit_code)
    if exit_code == 0:
        print("OK:",output)
    else:
        print("ERR:" , output)

except:
    print("ERR: Popen failed")


