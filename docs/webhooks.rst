Github Webhook Integration
==========================

Introduction
------------

Many CTF organizers will want to store their problems in a Github repository to make it easier to collaborate on problem writing. And since OpenCTF is run over Docker containers, adding problems to the platform can be a pain. A *webhook* is an endpoint that will cause the platform to automatically pull from a certain repository and add the problems in that repository to the database. This webhook can be triggered whenever someone pushes to the repository.

Setup
-----

To enable this feature, make sure you are logged in with a user with administrative privileges, and head over to **CTF Settings** under the **Admin** dropdown in the navigation bar. Under the various panels, you will see one panel labeled *Github Webhook*. It will look something like this.

|Webhook Configuration Panel|

Come up with a secret that you will use to verify Github’s payloads. Type this secret into the box, hit **Save Settings**. By adding a secret, you have enabled listening for webhooks on your platform. Now, you must create the webhook on Github. In your Github repository’s settings, go to the tab on the left labeled **Webhooks & services**. Click **Add webhook**. You’ll get to a page like this:

|New Webhook Page|

-  For **Payload URL**, put ``http://<your_domain>/api/admin/webhook``,
   replacing ``<your_domain>`` with your actual domain.
-  Leave **Content type** as ``application/json``.
-  Type in the secret that you created earlier for **Secret**.
-  Ideally, you should leave **triggers** as just ``push`` events, but
   it’s up to you to change that value as you see fit.
-  Obviously, you’d want your webhook to be **Active**.

With this, your platform and Github know about each others’ existence. But we’re not done. Most likely, the Github repository that you use for storing your problems will be private so random stalkers can’t access your competition problems before the competition begins. Therefore, the platform will need authentication to be able to pull from your repository.

The platform is able to authenticate to Github through SSH keys, so you need to add the platform’s *public key* to your repository. It’s important to note that you are adding it to the repository only, and not your account. First, locate the public key in the **CTF Settings** screen from earlier where you created the secret. You should now see the platform’s public key displayed. Then, go to your Github repository’s settings page again, and this time, go to the **Deploy keys** tab instead. Upon clicking **Add deploy key**, you’ll see a screen like this:

|New Deploy Key|

For **Title**, you may use whatever you’d like. For **Key**, paste the public key you found on the **CTF Settings** page. Leave the **Allow write access** box unchecked; the platform won’t be needing to write to your repository.

If you followed all the steps correctly, your platform should be ready to pull problems from your Github repository! Read on to learn about the format of your problems.

.. |Webhook Configuration Panel| image:: http://i.imgur.com/xntZNns.png
.. |New Webhook Page| image:: http://i.imgur.com/HH4KpEN.png
.. |New Deploy Key| image:: http://i.imgur.com/GDVIzvX.png