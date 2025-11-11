// ---------------------------------------------------- //
// This file is auto-generated                          //
// Manual changes made to this file will be overwritten //
// ---------------------------------------------------- //

#include <dependencies.plc.h>

{% if global_variables %}
// Global Variables
{{ global_variables }}
{% endif %}

{% if user_defined_data_types %}
// User-defined data types
{{ user_defined_data_types }}
{% endif %}

{% if functions %}
// Functions, Function Blocks and Programs
{{ functions }}
{% endif %}
