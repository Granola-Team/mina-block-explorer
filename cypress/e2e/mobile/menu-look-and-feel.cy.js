
const pages = require('../../pages.json');

describe('mobile menu look and feel', () => {
    pages.forEach(page => {
        it(`opens and closes the mobile menu on the ${page} page`, () => {
            cy.viewport('iphone-xr');
            cy.visit(page);
            cy.openMobileMenu();
            cy.get('nav').screenshot();
        });
    });

    it('has a header',() => {
        cy.viewport('iphone-xr');
        cy.visit("/");
        cy.get('header').screenshot();
    })
    
  })