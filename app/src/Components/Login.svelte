<!-- <script lang="ts">
/**
 * Helper function for POSTing data as JSON with fetch.
 *
 * @param {Object} options
 * @param {string} options.url - URL to POST data to
 * @param {FormData} options.formData - `FormData` instance
 * @return {Object} - Response body from URL that was POSTed to
 */
 async function postFormDataAsJson({ url, formData }) {
	const plainFormData = Object.fromEntries(formData.entries());
	const formDataJsonString = JSON.stringify(plainFormData);

	const fetchOptions = {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
			Accept: "application/json",
		},
		body: formDataJsonString,
	};

	const response = await fetch(url, fetchOptions);

	if (!response.ok) {
		const errorMessage = await response.text();
		throw new Error(errorMessage);
	}

	return response.json();
}

/**
 * Event handler for a form submit event.
 *
 * @see https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/submit_event
 *
 * @param {SubmitEvent} event
 */
async function handleFormSubmit(event) {
	event.preventDefault();

	const form = event.currentTarget;
	const url = form.action;

	try {
		const formData = new FormData(form);
		const responseData = await postFormDataAsJson({ url, formData });

		console.log({ responseData });
	} catch (error) {
		console.error(error);
	}
}

const exampleForm = document.getElementById("form");
exampleForm.addEventListener("submit", handleFormSubmit);
</script> -->

<form action="/api/users/login" id="form">
    <label for="fname">Username</label>
    <input type="text" id="fname" name="username" placeholder="Input Username"/>

    <label for="fpass">Password</label>
    <input type="password" id="fpass" name="password" placeholder="Input Password"/>

    <input type="submit" value="Submit" id = "form-submit"/>
</form>

<style>
    form {
        background-color:white;
        border: 2px solid black;
        padding: 30px;
        width:30%;
    }

    input[type="text"], input[type="password"] {
        width: 100%;
        padding: 12px 20px;
        margin: 14px 0;
        display: inline-block;
        border: 1px solid rgb(175, 170, 170);
        border-radius: 4px;
        box-sizing: border-box;
    }

    input[type="submit"] {
        width: 100%;
        background-color: #3a3a3a;
        color: white;
        padding: 14px 20px;
        margin: 8px 0;
        border: none;
        border-radius: 4px;
        cursor: pointer;
    }

    input[type="submit"]:hover {
        background-color: #1f1f1f;
    }
</style>
