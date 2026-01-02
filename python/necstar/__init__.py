from . import necstar
from .necstar import *

__doc__ = necstar.__doc__
if hasattr(necstar, "__all__"):
    __all__ = necstar.__all__
