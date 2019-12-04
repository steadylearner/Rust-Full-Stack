/*eslint-disable */
const nameRegExp = /^[a-zA-ZàáâäãåąčćęèéêëėįìíîïłńòóôöõøùúûüųūÿýżźñçčšžÀÁÂÄÃÅĄĆČĖĘÈÉÊËÌÍÎÏĮŁŃÒÓÔÖÕØÙÚÛÜŲŪŸÝŻŹÑßÇŒÆČŠŽ∂ð ,.'-]+$/u;
/*eslint-disable */
const cpfRegExp = /^([0-9]){3}\.([0-9]){3}\.([0-9]){3}-([0-9]){2}$/;
const phoneRegExp = /\(([1-9]{2})\) (?:[2-8]|9[1-9])[0-9]{3}\-[0-9]{4}$/; // with ()

// const phoneRegExp = new RegExp("^\([1-9]{2}\) (?:[2-8]|9[1-9])[0-9]{3}\-[0-9]{4}$"); // without ()
// const phoneRegExp = /^((\\+[1-9]{1,4}[ \\-]*)|(\\([0-9]{2,3}\\)[ \\-]*)|([0-9]{2,4})[ \\-]*)*?[0-9]{3,4}?[ \\-]*[0-9]{3,4}?$/ without () International

export {
	nameRegExp,
	cpfRegExp,
	phoneRegExp,
};
