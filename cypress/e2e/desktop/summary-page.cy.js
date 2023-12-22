describe('summary page', () => {
  beforeEach(() => {
    cy.viewport('macbook-16');
})
  it('opens and closes the account overview', () => {
    cy.visit('/');
    cy.openAccountDialog('table tr:nth-of-type(2) a');
    cy.closeAccountDialog();
  })
})