
const pages = require('../../pages.json');

suite([],'mobile', () => {
    
    it(`has menu`, () => {
        cy.viewport('iphone-xr');
        cy.visit("/summary");
        cy.openMobileMenu();
        cy.get('nav').matchImageSnapshot(`mobile-nav`);
        cy.get('nav a[href^="/blocks"]').click();
        cy.get('nav').should('not.be.visible');
    });

    it('has header',() => {
        cy.viewport('iphone-xr');
        cy.visit("/");
        cy.get('header').matchImageSnapshot('homepage-header');
    })
    
  })