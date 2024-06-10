suite(["tier1"], "account dialog", () => {
  let pages_with_account_dialog = ["/blocks"];

  let columns = ["Block Producer", "Coinbase Receiver"];

  pages_with_account_dialog.forEach((page) => {
    columns.forEach((col) =>
      it(`is launched on ${page} by click link in '${col}'`, () => {
        cy.visit(page);
        cy.wait(1000);
        cy.openAccountDialog(1, col, "Blocks");
        cy.closeAccountDialog();
      }),
    );
  });
});
