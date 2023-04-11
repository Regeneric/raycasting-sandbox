function dist(ax, ay, bx, by) result(res)
    real, intent(in) :: ax, ay, bx, by
    real             :: res

    res = sqrt(((bx-ax)*(bx-ax)) + ((by-ay)*(by-ay)))
end function