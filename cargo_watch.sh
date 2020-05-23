
#!/bin/bash

if [ ! -z "$1" ]  && [ $1 == '--container' ] 
then
    cargo watch -x 'test --target-dir /tmp/target'
else
    cargo watch -x test
fi