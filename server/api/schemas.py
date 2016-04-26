import re

from voluptuous import Required, Length, Schema, Invalid, MultipleInvalid
from decorators import WebException

def check(*callback_tuples):
	def v(value):
		for callbacks, msg in callback_tuples:
			for callback in callbacks:
				try:
					result = callback(value)
					if not result and type(result) == bool:
						raise Invalid(msg)
				except Exception:
					raise WebException(msg)
		return value
	return v

def verify_to_schema(schema, data):
	try:
		schema(data)
	except MultipleInvalid as error:
		raise WebException(str(error))