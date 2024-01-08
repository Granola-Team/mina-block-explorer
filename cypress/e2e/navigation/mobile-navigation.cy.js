const pages = require('../../pages.json');

describe('mobile menu navigation', () => {

    beforeEach(() => {
        cy.viewport('iphone-xr');
        cy.visit('/');
        cy.openMobileMenu();
    });

    pages.forEach(url => it(`navigates to ${url}`, () => { 
        cy.get(`nav a[href^="${url}"]`).click();
        cy.url().should('contain', url);
    }))
    
  })