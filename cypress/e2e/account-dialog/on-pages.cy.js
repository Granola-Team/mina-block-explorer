describe('account dialog', () => {
   
    let pages_with_account_dialog = [
      '/summary',
      '/blocks'
    ];

    pages_with_account_dialog.forEach(page => it(`is mounted on ${page}`,() => {
      cy.visit(page);
      cy.openAccountDialog(1, 'Block Producer');
      cy.closeAccountDialog();
    }))
});
