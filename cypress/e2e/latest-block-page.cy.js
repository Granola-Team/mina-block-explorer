describe('blocks page', () => {
    it('opens and closes the account overview on the summary page', () => {
        cy.visit('http://localhost:5274/blocks');
        cy.accountOverviewDialog('table tr:nth-of-type(2) a');
    })
  })