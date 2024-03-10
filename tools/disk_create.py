"""
JSON encoded disk creation tool.
"""

import json
import os
import sys

# iterate over all the files in the directory (excluding subdirectories)
directory = sys.argv[1] 
files = []
for filename in os.listdir(directory):
    if not os.path.isdir(directory + filename):
        with open(directory + filename, 'r') as file:
            data = file.read()
            print(data)
            # create the JSON object
            disk = {
                "name": filename,
                "contents": data
            }
            # now add the file to the list
            files.append(disk)


# create the JSON object
with open(sys.argv[2], 'w') as file:
    json.dump(files, file)
    print("Disk created successfully")