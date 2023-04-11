function degrees(ang) result(deg)
    real, intent(in) :: ang
    real, parameter  :: PI = 3.14159265

    deg = ang * 180.0/PI
end function