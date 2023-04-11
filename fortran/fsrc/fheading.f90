function heading_x(ang) result(res)
    real, intent(in) :: ang
    res = cos(ang)
end function

function heading_y(ang) result(res)
    real, intent(in) :: ang
    res = sin(ang)
end function