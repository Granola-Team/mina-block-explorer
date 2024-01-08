
const pages = require('../../pages.json');

describe('mobile menu look and feel', () => {
    
    it(`opens and closes the mobile menu`, () => {
        cy.viewport('iphone-xr');
        cy.visit("/summary");
        cy.openMobileMenu();
        cy.get('nav').matchImageSnapshot(`mobile-nav`);
        cy.get('nav a[href^="/blocks"]').click();
        cy.get('nav').should('not.be.visible');
    });

    it('has a header',() => {
        cy.viewport('iphone-xr');
        cy.visit("/");
        cy.get('header').matchImageSnapshot('homepage-header');
    })
    
  })