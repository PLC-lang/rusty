// Forward declaration of structs
{% for (name, _type) in types %}

    {% if _type.needs_forward_declaration() %}
        _type.forward_declare()
        struct {% _type.get_name() %};
    {% endif%}

{% endfor %}

// Declaration of non-struct data types
{% for (name, _type) in types %}

    {% if !_type.is_struct() %}
        {% _type.reference %} {% name %};
    {% endif%}

{% endfor %}

// Declaration of struct data types
{% for (name, _type) in types %}

    typedef struct {
        {% for var in _type.members %}
        {% _type.reference %} {% name %};
        {% endfor %}
    } {% name% };
    {% if _type.is_struct() %}
        _type.declare()
    {% endif%}

{% endfor %}

TYPE {% name %}

// Declaration of global variables
{% for (name, var) in variables %}
    var.declare()
{% endfor %}

// Declaration of functions


