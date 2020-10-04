# Copyright (C) 2020  Robert Chacón
# This file is part of LibreQoS.
#
# LibreQoS is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 2 of the License, or
# (at your option) any later version.
#
# LibreQoS is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with LibreQoS.  If not, see <http://www.gnu.org/licenses/>.
#
#            _     _ _               ___       ____  
#           | |   (_) |__  _ __ ___ / _ \  ___/ ___| 
#           | |   | | '_ \| '__/ _ \ | | |/ _ \___ \ 
#           | |___| | |_) | | |  __/ |_| | (_) |__) |
#           |_____|_|_.__/|_|  \___|\__\_\\___/____/
#                           v.0.4-alpha
#
import requests
from ispConfig import orgUNMSxAuthToken, unmsBaseURL, deviceModelBlacklistEnabled

#To omit bridged CPEs from shaping
if deviceModelBlacklistEnabled:
	deviceModelBlacklist = ['LBE-5AC-Gen2', 'LBE-5AC-Gen2', 'LBE-5AC-LR', 'AF-LTU5', 'AFLTULR', 'AFLTUPro', 'LTU-LITE']
else:
	deviceModelBlacklist = []

def pullUNMSCustomers():
	url = unmsBaseURL + "/nms/api/v2.1/sites?type=client&ucrm=true&ucrmDetails=true"
	headers = {'accept':'application/json', 'x-auth-token': orgUNMSxAuthToken}
	r = requests.get(url, headers=headers)
	jsonData = r.json()
	#print(jsonData)
	unmsCustomers = []
	for unmsClientSite in jsonData:
		try:
			downloadSpeedMbps = int(round(unmsClientSite['qos']['downloadSpeed']/1000000))
			uploadSpeedMbps = int(round(unmsClientSite['qos']['uploadSpeed']/1000000))
			address = unmsClientSite['description']['address']
			unmsClientSiteID = unmsClientSite['id']
			deviceList = getUNMSclientSiteDevices(unmsClientSiteID)
			thisCustomer = {
			'address'		:	address,
			'downloadSpeed'	:	downloadSpeedMbps,
			'uploadSpeed'	:	uploadSpeedMbps,
			}
			for device in deviceList:
				thisCustomer['deviceIPs'] = deviceList
			if deviceList:
				unmsCustomers.append(thisCustomer)
				print("Imported " + address)
		except:
			print("Failed to import customer " + unmsClientSite['description']['address'])
	return unmsCustomers

def getUNMSclientSiteDevices(siteID):
	url = unmsBaseURL + "/nms/api/v2.1/devices?siteId=" + siteID
	headers = {'accept':'application/json', 'x-auth-token': orgUNMSxAuthToken}
	r = requests.get(url, headers=headers)
	jsonData = r.json()
	deviceIPs = []
	for device in jsonData:
		deviceName = device['identification']['name']
		deviceMAC = device['identification']['mac']
		deviceIP = device['ipAddress']
		deviceModel = device['identification']['model']
		if not deviceModel:
			deviceModel = device['identification']['modelName']
		if deviceModel not in deviceModelBlacklist:
			deviceIPs.append(deviceIP)
			print("Added " + deviceModel + ":\t" + deviceName)
	return deviceIPs


















