
const pages = require('../../pages.json');

suite(["@VisualRegression"],'desktop', () => {
    
    it(`has menu in header`, () => {
        cy.visit("/summary");
        cy.get('header').matchImageSnapshot(`desktop-header`);
    });
    
});