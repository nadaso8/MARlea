# MARlea

A project aiming at revamping Aleae, a tool for "Stochastic Simulation of Molecular Reactions", written by Marc Riedel of the University of Minnesota Twin-Cities.
The original project is available here: https://cctbio.ece.umn.edu/aleae/

This is very much expirimental and just written for fun. Don't expect this to be completely accurate. We aren't biologists or chemists.

## about
A command line program for simulating DNA based chemical reaction networks.
            
This program takes some CSV as input and performs stochastic simulation on the provided system,
then prints results either to a specified output file or the command line.
            
Usage: Marlea `<QUERY>` `<INPUT_FILE>` `[Options]`
### Arguments:
 - `<QUERY>` - Specify the operation/query to perform. Possible values: \"settings\", \"validate\", \"simulate\", \"help\".
 - `<INPUT_FILE>` - Input file path to use.
            
### Options:
 -  -i, --init-file `<FILE_NAME>`      Specifies a file to read starting conditions from. By default if no file is specified the default is to assign species a count of 0, this is also the default for species not listed within the specified file. 
 -  -o, --output-file `<FILE_NAME>`    Specifies a file where the program should write its results. If omitted program will only print to the command line. By default this will simply use console output. 
 - --timeline `<FILE_NAME>`            **Advanced** Sets the program to write a full history of the solution at each step for all trials... This can easily be gigabytes of data.
 -  -t, --num-trials `<NUM_TRIALS>`    Specifies the number of times the simulation should simulate the chemical reaction network. By default this is 100 trials
 -  -r, --max-runtime `<MAX_RUNTIME>`  Specifies the maximum time the simulation is allowed to run for in seconds. By default runtime is unbounded
 -  -s,                                **Advanced** sets how tollerant each simulation will be of semi stable states. By defualt each trial will terminate after being semi stable for 100 steps. 
 
 ## input syntax 
 ### reactants/products
 `<coefficient> <species name>`
 - Note: If coefficient is ommited it defaults to 1
 ### reaction
 `<reactant + <reactant> => <product> + <product>, <reaction_rate>`
 - Note: there may be any number of reactants or products including 0 
 - Note: recursive inputs such as `my_var => my_var + my_other_var, some_rate` will cause the program to hang if no reaction consumes my_var elsewhere in the network for a reaction which may occure at all times please simply use null reactants such as ` => my_other_var, some_rate` this will not hang. 
 ## init syntax 
 `<species_name>, <initial_count>\n`
 - Note: only non zero values must be specified in init
