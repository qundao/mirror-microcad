# Workbench

[![test](.test/marker_cannot_deduced_output.svg)](.test/marker_cannot_deduced_output.log)

```µcad,marker_cannot_deduced_output
op operation() {
    std::geo2d::Circle(r=1mm);
    @input;
}

std::geo2d::Circle(r=1mm).operation();
```

[![test](.test/marker_wrong_output.svg)](.test/marker_wrong_output.log)

```µcad,marker_wrong_output
op operation() {  
  @input;  
  std::geo3d::Sphere(r=1mm)  
}  
  
std::geo2d::Circle(r=1mm).operation();
```
