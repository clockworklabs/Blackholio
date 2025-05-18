# addons/spacetimedb_client/DataDecompressor.gd
class_name DataDecompressor extends RefCounted

var _last_error: String = ""

# --- Error Handling ---

func has_error() -> bool:
	return _last_error != ""

func get_last_error() -> String:
	var err := _last_error
	_last_error = "" 
	return err

func _set_error(msg: String):
	if _last_error == "": 
		_last_error = "DataDecompressor Error: " + msg
		printerr(_last_error)

# --- Decompression Logic ---
func decompress(compressed_bytes: PackedByteArray, type: SpacetimeDBConnection.CompressionPreference) -> PackedByteArray:
	_last_error = "" 

	if compressed_bytes.is_empty():
		return PackedByteArray()

	var decompressed_data: PackedByteArray

	match type:
		SpacetimeDBConnection.CompressionPreference.NONE:
			return compressed_bytes

		SpacetimeDBConnection.CompressionPreference.BROTLI:
			_set_error("Brotli not supported")
			return []
		
		
		SpacetimeDBConnection.CompressionPreference.GZIP:
			#_set_error("GZip not supported")
			#return []
			
			## TODO: Doesnt work. Need chunk reading logic
			compressed_bytes = compressed_bytes.slice(1) #Remove msg compression tag
			if compressed_bytes.is_empty():
				_set_error("Gzip data became empty after slicing the compression tag.")
				return []

			print("Start (after slice): ", compressed_bytes.hex_encode()) 

			var gzip_stream := StreamPeerGZIP.new()
			
			var start_status = gzip_stream.start_decompression(true)
			if start_status != OK:
				_set_error("Failed to start Gzip decompression (Error: %s)" % error_string(start_status))
				return []
				
			var put_status = gzip_stream.put_partial_data(compressed_bytes)
			if put_status[0] != OK:
				_set_error("Failed to put data into Gzip stream (Error: %s)" % error_string(put_status[0]))
				return []
				
			var finish_status = gzip_stream.finish()
			if finish_status != OK:
				_set_error("Failed to finish Gzip stream (Error: %s). Input might be corrupted or incomplete." % error_string(finish_status))
				return []
				
			var available_bytes = gzip_stream.get_available_bytes()
			if available_bytes == 0 and finish_status == OK:
				push_warning("Gzip decompression finished successfully but produced 0 bytes.")
				return PackedByteArray() # Возвращаем пустой массив

			var get_status_data = gzip_stream.get_data(available_bytes)
			if get_status_data[0] != OK:
				_set_error("Failed to get decompressed Gzip data (Error: %s after finish)." % error_string(get_status_data[0]))
				return []
			print("GZIP WORK!")
			return get_status_data[1]

		_:
			_set_error("Unknown compression type requested: %d" % type)
			return []
	return []
