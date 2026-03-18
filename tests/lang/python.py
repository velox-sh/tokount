# line comment

def foo():
    x = "contains # not a comment"
    y = '# also not a comment'
    # real comment
    return x + y


def bar():
    """
    triple-quoted string (not a comment — treated as code)
    spanning multiple lines
    """
    z = """another
    multi-line
    string"""
    return z
