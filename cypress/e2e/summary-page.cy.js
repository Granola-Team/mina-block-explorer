describe('summary page', () => {
  it('opens and closes the account overview', () => {
    cy.visit('http://localhost:5274');
    cy.openCloseAccountDialog('table tr:nth-of-type(2) a');
  })
})