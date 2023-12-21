describe('navigate to account page', () => {
    it('navigates to account page', () => {
        cy.visit('http://localhost:5274/');
        cy.openAccountDialog('table tr:nth-of-type(2) a');
        cy.accountDialogToAccount();
    })
  })