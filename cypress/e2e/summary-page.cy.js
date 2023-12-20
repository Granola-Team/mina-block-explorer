describe('summary page', () => {
  it('opens and closes the account overview on the summary page', () => {
    cy.visit('http://localhost:5274');
    cy.accountOverviewDialog('table tr:nth-of-type(2) a');
  })
})