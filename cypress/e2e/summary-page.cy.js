describe('summary page', () => {
  it('opens and closes the account overview', () => {
    cy.visit('http://localhost:5274');
    cy.openAccountDialog('table tr:nth-of-type(2) a');
    cy.closeAccountDialog();
  })
})