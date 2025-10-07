# MNSF_Dashoard_Rust_Update
Same as the MNSF_Dashboard repo but backend code updated in rust and updated app structure

# Important
- If compiling code on a smaller machine like a ec2 t2.micro use the safe_compile.sh script to add a 4gb swap and limit compilation to 1 crate at a time, this will massivly increase compile time but stop the server from crashing

# Changes
- No longer 1 control script, each exchange independantly pushes raw data to DB
- All raw data form exchanges are permiated in DB so we can always recover or repair
- Everything should be modular, easier to update individual exchange or add new ones, also remvoves risk of 1 error creating a chain effect and breaking the entire app

# To Do
- Update exchange scripts into rust - Done
- Connect exchanges to mongoDB and start streaming data - Pending
- Update calcuulation and alert scripts - Pending
- Update PHP to display data - Pending
- Update server to run update - Pending

# Data Flow
- Read from exchange, saving raw calls to DB
- Sanatise and format exchange data from DB
- Each exchange struct should run endpoint call asynchronously (ensure request limits to not overload the server)
- Use systemd to running the polling scripts for each exchange
