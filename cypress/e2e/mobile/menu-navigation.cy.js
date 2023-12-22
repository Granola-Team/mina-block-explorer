describe('blocks page', () => {
    it('opens and closes the mobile menu', () => {
        cy.viewport('iphone-xr')
        cy.visit('/');
        cy.get('nav').should('not.be.visible');
        cy.get('label[for="nav-toggle"]').click();
        cy.get('nav').should('be.visible');
        cy.get('nav a[href*="blocks"]').click();
        cy.get('nav').should('not.be.visible');
        cy.url().should('contain', '/blocks')
    })
  })