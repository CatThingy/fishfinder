import numpy

def random_fish(height: int, width: int, min_scale: float, max_scale: float) -> fish_output: 
    """
    Generates a white image with the given width and height with a randomly
    rotated fish randomly scaled between `min_scale` and `max_scale`.
    """
    ...

class fish_output:
    @property
    def image(self) -> numpy.ndarray:
        """
        A 3-dimensional array of bytes reprsenting the RGB image of a fish.
        Has the shape (width, height, 3).
        """
        ...

    @property
    def fish_x(self) -> float:
        """
        The x-position of the top left corner of the fish.
        This value is in relative coordinates, where 0.0 is the left edge and 1.0 is the right edge.
        """
        ...

    @property
    def fish_y(self) -> float:
        """
        The y-position of the top left corner of the fish.
        This value is in relative coordinates, where 0.0 is the top edge and 1.0 is the bottom edge.
        """
        ...

    @property
    def fish_width(self) -> float:
        """
        The width of the fish.
        This value is in relative coordinates, where 1.0 is the full width of the image.
        """
        ...

    @property
    def fish_height(self) -> float:
        """
        The height of the fish.
        This value is in relative coordinates, where 1.0 is the full width of the image.
        """
        ...
