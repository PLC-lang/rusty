// ---------------------------------------------------- //
// This file is auto-generated                          //
// Manual changes made to this file will be overwritten //
// ---------------------------------------------------- //

#ifndef {{ file_name_caps }}
#define {{ file_name_caps }}

#include <stdint.h>
#include <math.h>
#include <stdbool.h>
#include <time.h>
#include <dependencies.plc.h>

#ifdef __cplusplus
extern "C" {
#endif
{% raw %}
{% endraw %}

{#- Aliases -#}
{% for alias in user_defined_types.aliases -%}
typedef {{ format_variable_for_definition(variable=alias) }};
{% raw %}
{% endraw %}
{%- endfor %}

{#- Enums -#}
{% for enum in user_defined_types.enums -%}
typedef enum e{{ enum.name }} {
    {% for variable in enum.variables -%}
    {{ format_variable_for_enum_definition(variable=variable) }}
    {%- if loop.last == false -%}
    {% raw %},
    {% endraw %}
    {%- endif -%}
    {% endfor -%}
{% raw %}
}{% endraw %} {{ enum.name }};
{% raw %}
{% endraw %}
{%- endfor %}

{#- Structs -#}
{% for struct in user_defined_types.structs -%}
typedef struct {
    {% for variable in struct.variables -%}
    {{ format_variable_for_definition(variable=variable) }}
    {%- if loop.last == false -%}
    {% raw %};
    {% endraw %}
    {%- endif -%}
    {%- if loop.last == true -%}
    {% raw %};{% endraw %}
    {%- endif -%}
    {% endfor -%}
{% raw %}
}{% endraw %} {{ struct.name }};
{% raw %}
{% endraw %}
{%- endfor %}

{#- Global Variables -#}
{% for global_variable in global_variables -%}
extern {{ format_variable_for_definition(variable=global_variable) }};
{% raw %}{% endraw %}
{%- if loop.last == true -%}
{% raw %}
{% endraw %}
{%- endif -%}
{%- endfor %}

{#- Functions -#}
{% for function in functions -%}
{{ function.return_type }} {{ function.name }}(
    {%- for parameter in function.parameters -%}
        {{- format_variable_for_parameter(variable=parameter) -}}
        {%- if loop.last == false -%}
            {% raw %}, {% endraw %}
        {%- endif -%}
    {%- endfor -%});
{% raw %}
{% endraw %}
{%- endfor -%}

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* !{{ file_name_caps }} */
