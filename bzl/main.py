import sys

output_path = sys.argv[1]
stuff_that_goes_into_the_file = sys.argv[2]

f = open(output_path, "w")
f.write(stuff_that_goes_into_the_file)
f.close()