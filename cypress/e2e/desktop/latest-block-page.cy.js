describe('blocks page', () => {
    it('opens and closes the account overview', () => {
        cy.visit('/blocks');
        cy.openAccountDialog('table tr:nth-of-type(2) a');
        cy.closeAccountDialog();
    })
  })