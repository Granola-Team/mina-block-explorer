describe('mobile menu navigation', () => {
    it('opens and closes the mobile menu', () => {
        cy.viewport('iphone-xr');
        cy.visit('/');
        cy.openMobileMenu();
        cy.get('nav a[href*="blocks"]').click();
        cy.get('nav').should('not.be.visible');
        cy.url().should('contain', '/blocks')
    })
  })