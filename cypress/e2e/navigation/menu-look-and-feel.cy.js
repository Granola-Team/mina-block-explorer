
const pages = require('../../pages.json');

describe('desktop menu look and feel', () => {
    
    it(`captures desktop menu`, () => {
        cy.visit("/");
        cy.get('header').matchImageSnapshot(`desktop-header`);
    });
    
    
});