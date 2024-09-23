#!/bin/bash

input_file=`dirname $0`"/../src/xsd/atomic_type.rs"
# Flag to indicate if we are inside the AtomicType enum
inside_enum=false

# Read the file line by line
while IFS= read -r line; do
    # Check if we found the start of the enum
    if [[ "$line" =~ pub\ enum\ AtomicType ]]; then
        inside_enum=true
        continue
    fi

    # Check if we reached the end of the enum
    if [[ "$line" =~ ^\s*} ]]; then
        inside_enum=false
        break
    fi

    # If we are inside the enum, extract the variants
    if $inside_enum; then
        # Use grep to find lines that look like enum variants
        entity =$( "$line" | grep -E '^\s*[A-Z][a-zA-Z0-9_]*,' | awk '{print $1}' | sed 's/,$//' )
        cat << EOF
            if atomic_type
        EOF
    fi
done < "$input_file"
