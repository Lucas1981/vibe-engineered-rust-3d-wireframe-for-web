function createRing(
  sides,
  turns,
  thickness,
  reach,
  startAngle,
  strokeColor,
  fillColor,
) {
  var i, j;
  var crd = new Array();
  var oldcrd = new Array();
  var object = new Objects_Class();
  var polygonCounter = 0;

  object.poly = new Array();

  for (j = 0; j < turns; j++) {
    oldcrd[0] = reach * Math.sin(Math.floor((360 / turns) * j) * degToRad);
    oldcrd[1] = reach * Math.cos(Math.floor((360 / turns) * j) * degToRad);
    oldcrd[2] = 0;

    for (i = 0; i < sides; i++) {
      crd[0] = 0;
      crd[1] =
        thickness *
        Math.sin((Math.floor((360 / sides) * i) - startAngle) * degToRad);
      crd[2] =
        thickness *
        Math.cos((Math.floor((360 / sides) * i) - startAngle) * degToRad);

      /* Get the right angle in and store the vertex -*/
      object.vlist[j * sides + i] = addVector(
        oldcrd,
        scaleAndRotate(
          crd[0],
          crd[1],
          crd[2],
          0,
          0,
          360 - (360 / turns) * j,
          1,
        ),
      );

      /* Store next polygon */
      if (j != 0 && i != 0) {
        object.poly[polygonCounter] = new Polygon_Class();
        object.poly[polygonCounter].vert[0] = (j - 1) * sides + (i - 1);
        object.poly[polygonCounter].vert[1] = (j - 1) * sides + i;
        object.poly[polygonCounter].vert[2] = j * sides + i;
        object.poly[polygonCounter].vert[3] = j * sides + (i - 1);
        if (fillColor == "random")
          object.poly[polygonCounter].fillColor = [
            Math.floor(Math.random() * 360),
            Math.floor(Math.random() * 100),
            Math.floor(Math.random() * 100),
          ];
        else object.poly[polygonCounter].fillColor = fillColor;

        object.poly[polygonCounter].strokeColor = strokeColor;
        polygonCounter++;
      }

      /* catch the last polygon point to tie the loop*/
      if (j != 0 && i == sides - 1) {
        object.poly[polygonCounter] = new Polygon_Class();
        object.poly[polygonCounter].vert[0] = (j - 1) * sides + i;
        object.poly[polygonCounter].vert[1] = (j - 1) * sides;
        object.poly[polygonCounter].vert[2] = j * sides;
        object.poly[polygonCounter].vert[3] = j * sides + i;
        if (fillColor == "random")
          object.poly[polygonCounter].fillColor = [
            Math.floor(Math.random() * 360),
            Math.floor(Math.random() * 100),
            Math.floor(Math.random() * 100),
          ];
        else object.poly[polygonCounter].fillColor = fillColor;

        object.poly[polygonCounter].strokeColor = strokeColor;
        polygonCounter++;
      }
    }

    // Catch the last set of polygons
    if (j == turns - 1) {
      for (i = 1; i < sides; i++) {
        object.poly[polygonCounter] = new Polygon_Class();
        object.poly[polygonCounter].vert[0] = j * sides + (i - 1);
        object.poly[polygonCounter].vert[1] = j * sides + i;
        object.poly[polygonCounter].vert[2] = i;
        object.poly[polygonCounter].vert[3] = i - 1;
        if (fillColor == "random")
          object.poly[polygonCounter].fillColor = [
            Math.floor(Math.random() * 360),
            Math.floor(Math.random() * 100),
            Math.floor(Math.random() * 100),
          ];
        else object.poly[polygonCounter].fillColor = fillColor;

        object.poly[polygonCounter].strokeColor = strokeColor;
        polygonCounter++;
      }

      object.poly[polygonCounter] = new Polygon_Class();
      object.poly[polygonCounter].vert[0] = j * sides + sides - 1;
      object.poly[polygonCounter].vert[1] = j * sides;
      object.poly[polygonCounter].vert[2] = 0;
      object.poly[polygonCounter].vert[3] = sides - 1;
      if (fillColor == "random")
        object.poly[polygonCounter].fillColor = [
          Math.floor(Math.random() * 360),
          Math.floor(Math.random() * 100),
          Math.floor(Math.random() * 100),
        ];
      else object.poly[polygonCounter].fillColor = fillColor;

      object.poly[polygonCounter].strokeColor = strokeColor;
    }
  }

  return object;
}
