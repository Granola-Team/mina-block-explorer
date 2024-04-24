suite(["@VisualRegression"], "pill color", () => {
  let pills = {
    grey: {
      label: "Global",
      location:
        "/blocks/3NKRWwREPywcAGZa6JeMm3dKq9gri6152y4tY5zJ2bz794DJAZm5/spotlight",
      type: "spotlight",
    },
    blue: {
      label: "Global",
      location:
        "blocks?query=3NLss1SvtWDK2rUAjHFwF6LMtD8thLoNhcYNSckgqM9bQksUDLFa",
      type: "table",
      table_name: "Blocks",
      table_column: "User Commands",
    },
  };

  Object.entries(pills).forEach(([pill, obj]) => {
    const { label, location, type } = obj;
    it(`is correct for the ${pill} pill`, () => {
      cy.visit(location);
      cy.prepareSnapshotTest();

      switch (type) {
        case "spotlight":
          cy.spotlightData(label).as("data");
          break;
        case "table":
          const { table_name, table_column } = obj;
          cy.aliasTableValue(0, table_column, table_name, "data");
          cy.get("@data").scrollIntoView();
          break;
        default:
          throw new Error("Unmapped case");
      }

      cy.get("@data").matchImageSnapshot(`${pill}-pill`);
    });
  });
});
