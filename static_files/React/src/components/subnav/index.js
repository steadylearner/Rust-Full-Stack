import React from 'react';
import { GitHub } from "../reuse";

const Subnav = () => {
    const subnavclass = "fixed sub nav-height width-vw theme-black border-white center"

    return (
        <footer className={subnavclass} >
            <GitHub />
        </footer>
    )
}

export default Subnav;