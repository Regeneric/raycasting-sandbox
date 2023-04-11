function radians(ang) result(rad)
    real, intent(in) :: ang
    real, parameter  :: PI = 3.14159265

    rad = ang * (PI/180.0)
end function