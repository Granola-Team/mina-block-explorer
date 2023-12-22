describe('navigate to account page', () => {
    it('navigates to account page', () => {
        cy.visit('/');
        cy.openAccountDialog('table tr:nth-of-type(2) a');
        cy.accountDialogToAccount();
    })
  })